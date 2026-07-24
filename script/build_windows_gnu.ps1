param(
    [switch]$SkipBundleInstall,
    [switch]$SkipToolInstall,
    [switch]$BuildGem,
    [switch]$TestGem,
    [switch]$RunTests,
    [string]$SmokeUrl = $env:WREQ_SMOKE_URL
)

$ErrorActionPreference = "Stop"

$target = "x86_64-pc-windows-gnu"
$platform = "x64-mingw-ucrt"
$gemPattern = "pkg\wreq-*-$platform.gem"
if ([string]::IsNullOrWhiteSpace($SmokeUrl)) {
    $SmokeUrl = "https://httpbin.io/get"
}
$rubyRoot = Split-Path (Split-Path (Get-Command ruby).Source -Parent) -Parent
$ucrt = Join-Path $rubyRoot "msys64\ucrt64"
$ucrtBin = Join-Path $ucrt "bin"
$msysPackages = @(
    "mingw-w64-ucrt-x86_64-gcc",
    "mingw-w64-ucrt-x86_64-clang",
    "mingw-w64-ucrt-x86_64-cmake",
    "mingw-w64-ucrt-x86_64-pkgconf"
)

function Invoke-Step {
    param(
        [string]$Name,
        [scriptblock]$Command
    )

    Write-Host "==> $Name"
    # Native command failures do not honor ErrorActionPreference in Windows PowerShell.
    $global:LASTEXITCODE = 0
    & $Command
    if ($LASTEXITCODE -ne 0) {
        throw "$Name failed with exit code $LASTEXITCODE."
    }
}

function Get-MissingUcrtTools {
    $missing = @()

    if (-not (Test-Path (Join-Path $ucrtBin "gcc.exe")) -or -not (Test-Path (Join-Path $ucrtBin "g++.exe"))) {
        $missing += "gcc/g++"
    }

    if (-not (Test-Path (Join-Path $ucrtBin "clang.exe")) -or -not (Test-Path (Join-Path $ucrtBin "libclang.dll"))) {
        $missing += "clang/libclang"
    }

    if (-not (Test-Path (Join-Path $ucrtBin "cmake.exe"))) {
        $missing += "cmake"
    }

    if (-not (Test-Path (Join-Path $ucrtBin "pkgconf.exe")) -and -not (Test-Path (Join-Path $ucrtBin "pkg-config.exe"))) {
        $missing += "pkgconf"
    }

    $missing
}

function Get-LatestPlatformGem {
    $gem = Get-ChildItem -Path $gemPattern -ErrorAction SilentlyContinue |
        Sort-Object LastWriteTime -Descending |
        Select-Object -First 1

    if (-not $gem) {
        throw "No Windows GNU platform gem found at '$gemPattern'. Re-run with -BuildGem first."
    }

    $gem
}

function Invoke-RubySmoke {
    param(
        [string]$Name,
        [string]$Code
    )

    Write-Host "Smoke: $Name"
    ruby -e $Code
    if ($LASTEXITCODE -ne 0) {
        throw "$Name failed with exit code $LASTEXITCODE."
    }
}

Invoke-Step "Check Ruby platform" {
    $rubyArch = & ruby -rrbconfig -e "print RbConfig::CONFIG['arch']"
    if ($rubyArch -ne $platform) {
        throw "Expected Ruby platform '$platform', got '$rubyArch'. Use RubyInstaller UCRT for this build."
    }

    ruby -v
    ruby -rrbconfig -rrubygems -e "puts RbConfig::CONFIG.values_at('arch', 'host', 'CC').join(%q{ }); puts Gem::Platform.local"
}

Invoke-Step "Install Rust target" {
    $installedTargets = @(rustup target list --installed)
    if ($installedTargets -contains $target) {
        Write-Host "Rust target $target already installed; skipping rustup."
        return
    }

    rustup target add $target
    if ($LASTEXITCODE -ne 0) {
        throw "Failed to install Rust target $target."
    }
}

Invoke-Step "Check MSYS2 UCRT build tools" {
    $missing = @(Get-MissingUcrtTools)
    if ($missing.Count -eq 0) {
        Write-Host "MSYS2 UCRT build tools already present; skipping pacman."
        return
    }

    if ($SkipToolInstall) {
        throw "Missing MSYS2 UCRT build tools: $($missing -join ', '). Re-run without -SkipToolInstall or install them with ridk."
    }

    Write-Host "Missing MSYS2 UCRT build tools: $($missing -join ', ')"
    ridk exec pacman -S --needed --noconfirm @msysPackages
    if ($LASTEXITCODE -ne 0) {
        $stillMissing = @(Get-MissingUcrtTools)
        if ($stillMissing.Count -eq 0) {
            Write-Warning "pacman returned exit code $LASTEXITCODE, but all required tools are present; continuing."
            $global:LASTEXITCODE = 0
            return
        }

        throw "Failed to install MSYS2 UCRT build tools. Missing: $($stillMissing -join ', '). If pacman cannot lock its database, close other pacman/ridk shells, run PowerShell as Administrator, or install RubyInstaller in a user-writable directory."
    }
}

Invoke-Step "Configure Windows GNU toolchain" {
    $env:CARGO_BUILD_TARGET = $target
    $env:LIBCLANG_PATH = $ucrtBin
    $env:CC_x86_64_pc_windows_gnu = Join-Path $ucrtBin "gcc.exe"
    $env:CXX_x86_64_pc_windows_gnu = Join-Path $ucrtBin "g++.exe"
    $env:CMAKE = Join-Path $ucrtBin "cmake.exe"
    Remove-Item Env:\CMAKE_GENERATOR -ErrorAction SilentlyContinue

    rustc -Vv
    Write-Host "LIBCLANG_PATH=$env:LIBCLANG_PATH"
    Write-Host "CC_x86_64_pc_windows_gnu=$env:CC_x86_64_pc_windows_gnu"
    Write-Host "CXX_x86_64_pc_windows_gnu=$env:CXX_x86_64_pc_windows_gnu"
    Write-Host "CMAKE=$env:CMAKE"
}

if (-not $SkipBundleInstall) {
    Invoke-Step "Install Ruby dependencies" {
        bundle install
    }
}

Invoke-Step "Compile native extension" {
    ridk exec ruby -S bundle exec rake compile
}

Invoke-Step "Sync versioned extension" {
    $rubyMinor = & ruby -e "print RUBY_VERSION[/\A\d+\.\d+/]"
    $dest = "lib\wreq_ruby\$rubyMinor"
    New-Item -ItemType Directory -Force $dest | Out-Null
    Copy-Item "lib\wreq_ruby\wreq_ruby.so" (Join-Path $dest "wreq_ruby.so") -Force
}

Invoke-Step "Verify extension loads" {
    ruby -Ilib -rwreq -e "puts Wreq::VERSION; puts Wreq::Client.new.class"
}

if ($RunTests) {
    Invoke-Step "Run tests" {
        ridk exec ruby -S bundle exec rake test
    }
}

if ($BuildGem) {
    Invoke-Step "Build local platform gem" {
        $rubyMinor = & ruby -e "print RUBY_VERSION[/\A\d+\.\d+/]"
        $dest = "lib\wreq_ruby\$rubyMinor"
        New-Item -ItemType Directory -Force $dest | Out-Null
        Copy-Item "lib\wreq_ruby\wreq_ruby.so" (Join-Path $dest "wreq_ruby.so") -Force

        ruby script/build_platform_gem.rb $platform
    }
}

if ($TestGem) {
    Invoke-Step "Install and smoke test platform gem" {
        $gem = Get-LatestPlatformGem
        $gemHome = Join-Path ([System.IO.Path]::GetTempPath()) "wreq-gem-test-$PID"
        $gemBin = Join-Path $gemHome "bin"
        $testDir = Join-Path ([System.IO.Path]::GetTempPath()) "wreq-gem-test-cwd-$PID"

        Remove-Item -Recurse -Force $gemHome, $testDir -ErrorAction SilentlyContinue
        New-Item -ItemType Directory -Force $gemHome, $gemBin, $testDir | Out-Null

        $oldGemHome = $env:GEM_HOME
        $oldGemPath = $env:GEM_PATH
        $oldGemrc = $env:GEMRC
        $oldRubyopt = $env:RUBYOPT
        $oldRubylib = $env:RUBYLIB
        $oldBundleGemfile = $env:BUNDLE_GEMFILE
        $oldBundleBinPath = $env:BUNDLE_BIN_PATH
        $oldBundlerVersion = $env:BUNDLER_VERSION
        $oldSmokeUrl = $env:WREQ_SMOKE_URL
        $oldWreqGemHome = $env:WREQ_GEM_HOME

        try {
            $env:GEM_HOME = $gemHome
            $env:GEM_PATH = $gemHome
            $env:GEMRC = ""
            $env:RUBYOPT = ""
            $env:RUBYLIB = ""
            $env:BUNDLE_GEMFILE = ""
            $env:BUNDLE_BIN_PATH = ""
            $env:BUNDLER_VERSION = ""
            $env:WREQ_SMOKE_URL = $SmokeUrl
            $env:WREQ_GEM_HOME = $gemHome

            gem install --norc --local --install-dir $gemHome --bindir $gemBin --no-document --force --no-user-install $gem.FullName
            if ($LASTEXITCODE -ne 0) {
                throw "Failed to install $($gem.FullName)."
            }

            Push-Location $testDir
            try {
                Invoke-RubySmoke "Load installed platform gem" @'
STDOUT.sync = true
STDERR.sync = true
require "wreq"
spec = Gem.loaded_specs.fetch("wreq")
expected = File.expand_path(ENV.fetch("WREQ_GEM_HOME"))
actual = File.expand_path(spec.full_gem_path)
raise "loaded #{actual}, expected under #{expected}" unless actual.start_with?(expected)
puts "loaded #{spec.full_name}"
puts actual
puts "wreq #{Wreq::VERSION}"
'@

                Invoke-RubySmoke "Request through installed platform gem" @'
STDOUT.sync = true
STDERR.sync = true
require "wreq"
client = Wreq::Client.new
resp = client.get(ENV.fetch("WREQ_SMOKE_URL"), timeout: 20)
puts "HTTP #{resp.code}"
raise "unexpected status #{resp.code}" unless resp.code == 200
'@
            } finally {
                Pop-Location
            }
        } finally {
            $env:GEM_HOME = $oldGemHome
            $env:GEM_PATH = $oldGemPath
            $env:GEMRC = $oldGemrc
            $env:RUBYOPT = $oldRubyopt
            $env:RUBYLIB = $oldRubylib
            $env:BUNDLE_GEMFILE = $oldBundleGemfile
            $env:BUNDLE_BIN_PATH = $oldBundleBinPath
            $env:BUNDLER_VERSION = $oldBundlerVersion
            $env:WREQ_SMOKE_URL = $oldSmokeUrl
            $env:WREQ_GEM_HOME = $oldWreqGemHome
            Remove-Item -Recurse -Force $gemHome, $testDir -ErrorAction SilentlyContinue
        }
    }
}

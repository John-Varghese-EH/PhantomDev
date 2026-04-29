#!/bin/bash
# PhantomDev One-Click Installer
# This script installs PhantomDev on Linux, macOS, and Windows (via WSL)

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Print colored output
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Detect OS
detect_os() {
    case "$(uname -s)" in
        Linux*)     OS=Linux;;
        Darwin*)    OS=Mac;;
        CYGWIN*)    OS=Cygwin;;
        MINGW*)     OS=MinGW;;
        MSYS*)      OS=MSYS;;
        *)          OS="UNKNOWN:${uname -s}"
    esac
}

# Detect architecture
detect_arch() {
    case "$(uname -m)" in
        x86_64)     ARCH=x86_64;;
        aarch64)    ARCH=aarch64;;
        arm64)      ARCH=aarch64;;
        *)          ARCH="UNKNOWN:${uname -m}"
    esac
}

# Check if Rust is installed
check_rust() {
    if ! command -v cargo &> /dev/null; then
        print_warning "Rust/Cargo not found. Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source $HOME/.cargo/env
    else
        print_success "Rust/Cargo found: $(cargo --version)"
    fi
}

# Install PhantomDev from source
install_from_source() {
    print_info "Installing PhantomDev from source..."

    # Create temporary directory
    TEMP_DIR=$(mktemp -d)
    cd "$TEMP_DIR"

    # Clone repository
    print_info "Cloning repository..."
    git clone https://github.com/John-Varghese-EH/PhantomDev.git
    cd PhantomDev

    # Build release
    print_info "Building PhantomDev (this may take a few minutes)..."
    cargo build --release

    # Install binary
    print_info "Installing binary..."
    if [ "$OS" = "Linux" ] || [ "$OS" = "Mac" ]; then
        sudo cp target/release/phantomdev /usr/local/bin/
        sudo chmod +x /usr/local/bin/phantomdev
    else
        print_error "Unsupported OS for direct installation"
        exit 1
    fi

    # Cleanup
    cd -
    rm -rf "$TEMP_DIR"

    print_success "PhantomDev installed successfully!"
}

# Install from pre-built binary
install_from_binary() {
    print_info "Installing PhantomDev from pre-built binary..."

    VERSION="latest"
    DOWNLOAD_URL="https://github.com/John-Varghese-EH/PhantomDev/releases/latest/download"

    case "$OS" in
        Linux)
            if [ "$ARCH" = "x86_64" ]; then
                BINARY="phantomdev-linux-x86_64.tar.gz"
            else
                BINARY="phantomdev-linux-aarch64.tar.gz"
            fi
            ;;
        Mac)
            if [ "$ARCH" = "x86_64" ]; then
                BINARY="phantomdev-macos-x86_64.tar.gz"
            else
                BINARY="phantomdev-macos-aarch64.tar.gz"
            fi
            ;;
        *)
            print_error "Pre-built binary not available for $OS $ARCH"
            print_info "Falling back to source installation..."
            install_from_source
            return
            ;;
    esac

    # Download binary
    print_info "Downloading $BINARY..."
    curl -L "$DOWNLOAD_URL/$BINARY" -o /tmp/phantomdev.tar.gz

    # Extract and install
    print_info "Extracting and installing..."
    tar -xzf /tmp/phantomdev.tar.gz -C /tmp/
    sudo cp /tmp/phantomdev /usr/local/bin/
    sudo chmod +x /usr/local/bin/phantomdev

    # Cleanup
    rm -f /tmp/phantomdev.tar.gz /tmp/phantomdev

    print_success "PhantomDev installed successfully!"
}

# Verify installation
verify_installation() {
    print_info "Verifying installation..."

    if command -v phantomdev &> /dev/null; then
        VERSION=$(phantomdev --version 2>/dev/null || echo "unknown")
        print_success "PhantomDev $VERSION installed at $(which phantomdev)"
    else
        print_error "Installation verification failed"
        exit 1
    fi
}

# Initialize PhantomDev
initialize_phantomdev() {
    print_info "Initializing PhantomDev in current directory..."

    if [ -d ".phantomdev" ]; then
        print_warning "PhantomDev already initialized in this directory"
        return
    fi

    phantomdev init
    print_success "PhantomDev initialized!"
}

# Main installation flow
main() {
    echo "    /\\"
    echo "   /__\\    PhantomDev"
    echo "  /    \\   Humanizer for AI Agents & Commits"
    echo " /______\\  github.com/John-Varghese-EH/PhantomDev"
    echo ""

    detect_os
    detect_arch

    print_info "Detected OS: $OS"
    print_info "Detected Architecture: $ARCH"

    # Check Rust
    check_rust

    # Ask installation method
    print_info "Choose installation method:"
    echo "  1) Install from pre-built binary (recommended)"
    echo "  2) Install from source"
    read -p "Enter choice [1-2]: " choice

    case $choice in
        1) install_from_binary;;
        2) install_from_source;;
        *) print_error "Invalid choice"; exit 1;;
    esac

    # Verify installation
    verify_installation

    # Ask to initialize
    read -p "Initialize PhantomDev in current directory? [y/N]: " init_choice
    if [ "$init_choice" = "y" ] || [ "$init_choice" = "Y" ]; then
        initialize_phantomdev
    fi

    echo ""
    print_success "Installation complete!"
    echo ""
    echo "Quick start:"
    echo "  phantomdev scan          # Scan for AI-generated content"
    echo "  phantomdev humanize      # Humanize code"
    echo "  phantomdev score         # Check stealth score"
    echo "  phantomdev dashboard     # Launch TUI dashboard"
    echo ""
}

# Run main
main

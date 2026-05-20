# IAM Business - Car Rental Manager

A complete desktop car rental management application made using Rust and the egui framework. 
This platform eliminates external database dependencies and cloud overhead by utilizing localized flat-file CSV storage located directly within the host system user directories.

---

## System Features

### Dashboard Module
* **Operational Analytics:** Provides macro statistics cards detailing active fleet counts, current vehicle availability, and total accumulated revenue.
* **Status Monitors:** Implements visual status indicators for rapid fleet monitoring alongside a ledger displaying recent contractual activity.

### Availability Module
* **Temporal Query Engine:** Allows operators to select specific date ranges to instantaneously view unreserved fleet vehicles.
* **Streamlined Workflow:** Initiates a pre-filled contract pipeline directly from an available vehicle file card with a single click.

### Rental Initialization Module
* **Comprehensive Entry Ledger:** Captures mandatory operational vectors including vehicle identification, client credentials, contact records, staff agent tracking, temporal boundaries, and odometer metrics (initial/expected return).
* **Live Calculation Pipeline:** Computes dynamic live price calculations and runs algorithmic overlapping-date validation checks prior to contract submission.

### Contract Ledger Module
* **Tabular Visualization:** Implements a spreadsheet-style scrollable table displaying comprehensive structural data including duration metrics, distance tracking, and financial breakdowns.
* **Advanced Querying:** Provides real-time string searches and logical status filters to isolate specific records rapidly.
* **Execution Gates:** Features discrete confirmation buttons for immediate contract validation or termination.

### Fleet Management Module
* **Real-Time Tracking:** Monitors daily asset statuses (Rented/Free) across the entire fleet registry.
* **Inventory Control:** Integrated administrative interfaces to dynamically add new assets or delete retired units from production tracking.

---

## Data Persistence & Storage

To minimize deployment overhead and infrastructure costs, application data is persisted within native filesystem shares as plain CSV files. This enables immediate interoperability with spreadsheet applications like Microsoft Excel or LibreOffice.

| Data Type | Windows Environment | Linux / macOS Environment |
|:---|:---|:---|
| Fleet Records (`cars.csv`) | `%APPDATA%\IAMBusiness\cars.csv` | `~/.local/share/IAMBusiness/cars.csv` |
| Transaction Ledger (`rentals.csv`) | `%APPDATA%\IAMBusiness\rentals.csv` | `~/.local/share/IAMBusiness/rentals.csv` |

---

## Project Architecture

```text
iam-business/
├── Cargo.toml                  # Dependency declarations and optimization profiles
├── src/
│   └── main.rs                 # Comprehensive application codebase (~700 lines)
├── installer/
│   ├── windows/
│   │   └── setup.iss           # Inno Setup deployment script for Windows compilation
│   └── linux/
│       ├── iam-business.desktop# Desktop system integration file
│       └── create-deb.sh       # Native Debian packaging compilation script
├── build-windows.bat           # Automated single-command Windows build execution script
├── build-linux.sh              # Automated single-command Linux build execution script
└── dist/                       # Output directory for production distribution artifacts
    ├── windows/
    │   └── IAMBusiness-Setup.exe
    └── linux/
        └── iam-business_1.0.0_amd64.deb
```

##Compilation & Deployment

### Compilation Prerequisites
Ensure the native Rust toolchain is accessible within your host shell environment:
```bash
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
```

## Linux Platforms
Execute system dependency provisions based on your target system package manager before compilation:
```bash
# Package requirements for Fedora environments
sudo dnf install gcc libxcb-devel libX11-devel

# Package requirements for Ubuntu/Debian environments
sudo apt install build-essential libxcb1-dev libxcb-render0-dev \
                 libxcb-shape0-dev libxcb-xfixes0-dev

# Local compilation and immediate binary execution
cargo build --release
./target/release/iam-business

# Execution of distribution packaging sequence
chmod +x build-linux.sh
./build-linux.sh
sudo dpkg -i dist/linux/iam-business_1.0.0_amd64.deb
```

## Windows Platforms
Verify that both the Rust toolchain and Inno Setup 6 are provisioned on the host environment paths, then execute the automated build batch file:
```bash
build-windows.bat
```
The resulting distribution binary will be accessible at dist\windows\IAMBusiness-Setup.exe.

# IAM BUSINESS APP

<table>
  <tr>
    <td><img src="pics/1.png" alt="Dashboard" width="500"/></td>
    <td><img src="pics/2.png" alt="Contract Management" width="500"/></td>
  </tr>
  <tr>
    <td><img src="pics/3.png" alt="Fleet Management" width="500"/></td>
    <td><img src="pics/4.png" alt="Financial Report" width="500"/></td>
  </tr>
</table>

IAM Business is a desktop application developed in Rust using the eframe/egui framework. It is designed to assist with the management of car rental operations. The application provides tools for handling contracts, vehicle inventory, financial reporting, maintenance logs, vehicle sales tracking, and cash register management.

---

## Features

* **Contract Management:** Creation, tracking, and payment management for rental contracts.
* **Vehicle Availability:** Real-time visual dashboard for checking the status and availability of the fleet.
* **Fleet Management:** Tools to add, edit, and track the status of vehicles, including maintenance records.
* **Cash Register:** Dedicated module to track daily cash flow, including inflows (encaissements) and outflows (decaissements).
* **Financial Reporting:** High-level reporting on business performance and profitability.
* **Maintenance Logs:** Tracking of repairs and technical service history for each vehicle.
* **Car Sales:** Lifecycle management for vehicles, from acquisition to final sale, including profit calculations.
* **Cross-Platform:** Built with Rust, ensuring compatibility on both Windows and Linux.

---

## Tech Stack

* **Language:** Rust
* **GUI Framework:** eframe / egui (Immediate Mode)
* **Data Persistence:** serde / csv
* **Date Handling:** chrono

---

## Getting Started

### Prerequisites

* [Rust](https://www.rust-lang.org/tools/install) installed on your machine.

### Installation

1. Clone the repository:
   ```bash
   git clone [https://github.com/yourusername/iam-business.git](https://github.com/yourusername/iam-business.git)
    ``
2. Navigate to the project directory:
```bash
cd iam-business
```


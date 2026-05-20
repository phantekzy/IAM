; ─────────────────────────────────────────────────────────────────────────────
;  IAM Business – Inno Setup Script
;  Produces: IAMBusiness-Setup.exe
;  Tool:     Inno Setup 6  (https://jrsoftware.org/isinfo.php)
; ─────────────────────────────────────────────────────────────────────────────

[Setup]
AppName                 = IAM Business
AppVersion              = 1.0.0
AppPublisher            = IAM Business
AppPublisherURL         = https://iam.business
AppSupportURL           = https://iam.business
AppUpdatesURL           = https://iam.business
DefaultDirName          = {autopf}\IAM Business
DefaultGroupName        = IAM Business
AllowNoIcons            = yes
OutputDir               = ..\..\dist\windows
OutputBaseFilename      = IAMBusiness-Setup
Compression             = lzma2/ultra64
SolidCompression        = yes
WizardStyle             = modern
PrivilegesRequired      = admin
DisableProgramGroupPage = yes
UninstallDisplayIcon    = {app}\iam-business.exe
ChangesAssociations     = no

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"
Name: "french";  MessagesFile: "compiler:Languages\French.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
; Main binary (must be compiled first with: cargo build --release)
Source: "..\..\target\release\iam-business.exe"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\IAM Business";          Filename: "{app}\iam-business.exe"
Name: "{group}\Uninstall IAM Business"; Filename: "{uninstallexe}"
Name: "{autodesktop}\IAM Business";    Filename: "{app}\iam-business.exe"; Tasks: desktopicon

[Run]
Filename: "{app}\iam-business.exe"; Description: "{cm:LaunchProgram,IAM Business}"; Flags: nowait postinstall skipifsilent

[UninstallDelete]
; Data files are in %APPDATA%\IAMBusiness – preserve them on uninstall
Type: dirifempty; Name: "{app}"

[Code]
// ─── Check for Visual C++ Redistributable (optional) ─────────────────────────
function VCRedistNeedsInstall: Boolean;
begin
  Result := not RegKeyExists(HKLM, 'SOFTWARE\Microsoft\VisualStudio\14.0\VC\Runtimes\x64');
end;

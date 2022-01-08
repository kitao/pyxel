#define PyxelVersion "1.5.8"

[Setup]
AppName=Pyxel
AppVerName=Pyxel {#PyxelVersion}
AppVersion={#PyxelVersion}
ChangesAssociations=yes
ChangesEnvironment=true
DefaultDirName={localappdata}\Programs\Pyxel
DisableWelcomePage=No
OutputBaseFilename=pyxel-{#PyxelVersion}-windows-setup
OutputDir=..\dist
PrivilegesRequired=lowest
SetupIconFile=..\doc\images\pyxel_icon_64x64.ico
UninstallDisplayIcon={app}\unins000.exe,0

[Tasks]
Name: envPyxapp; Description: "Associate .pyxapp files with Pyxel"
Name: envPyxres; Description: "Associate .pyxres files with Pyxel"
Name: envPath; Description: "Add Pyxel to the user PATH"

[Files]
Source: "build\pyxel-{#PyxelVersion}-windows\*"; DestDir: "{app}"; Flags: recursesubdirs

[Registry]
Root: HKA; Subkey: "Software\Classes\.pyxapp\OpenWithProgids"; ValueType: string; ValueName: "Pyxel.pyxapp"; ValueData: ""; Flags: uninsdeletevalue; Tasks: envPyxapp
Root: HKA; Subkey: "Software\Classes\Pyxel.pyxapp"; ValueType: string; ValueName: ""; ValueData: "Pyxel Application File"; Flags: uninsdeletekey; Tasks: envPyxapp
Root: HKA; Subkey: "Software\Classes\Pyxel.pyxapp\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\unins000.exe,0"; Tasks: envPyxapp
Root: HKA; Subkey: "Software\Classes\Pyxel.pyxapp\shell\open\command"; ValueType: string; ValueName: ""; ValueData: "WScript ""{app}\pyxel.vbs"" play ""%1"""; Tasks: envPyxapp
Root: HKA; Subkey: "Software\Classes\Applications\pyxel.exe\SupportedTypes"; ValueType: string; ValueName: ".pyxapp"; ValueData: ""; Tasks: envPyxapp

Root: HKA; Subkey: "Software\Classes\.pyxres\OpenWithProgids"; ValueType: string; ValueName: "Pyxel.pyxres"; ValueData: ""; Flags: uninsdeletevalue; Tasks: envPyxres
Root: HKA; Subkey: "Software\Classes\Pyxel.pyxres"; ValueType: string; ValueName: ""; ValueData: "Pyxel Resource File"; Flags: uninsdeletekey; Tasks: envPyxres
Root: HKA; Subkey: "Software\Classes\Pyxel.pyxres\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\unins000.exe,0"; Tasks: envPyxres
Root: HKA; Subkey: "Software\Classes\Pyxel.pyxres\shell\open\command"; ValueType: string; ValueName: ""; ValueData: "WScript ""{app}\pyxel.vbs"" edit ""%1"""; Tasks: envPyxres
Root: HKA; Subkey: "Software\Classes\Applications\pyxel.exe\SupportedTypes"; ValueType: string; ValueName: ".pyxres"; ValueData: ""; Tasks: envPyxres

[Code]
procedure CurStepChanged(CurStep: TSetupStep);
var
    Path: string;
    Paths: string;
begin
    if (CurStep <> ssPostInstall) or (not WizardIsTaskSelected('envPath'))
    then exit;

    Path := ExpandConstant('{app}');

    if not RegQueryStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', Paths)
    then Paths := '';

    if Pos(';' + Uppercase(Path) + ';', ';' + Uppercase(Paths) + ';') > 0
    then exit;

    Paths := Paths + ';'+ Path +';'

    if RegWriteStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', Paths)
    then Log(Format('The [%s] added to PATH: [%s]', [Path, Paths]))
    else Log(Format('Error while adding the [%s] to PATH: [%s]', [Path, Paths]));
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
var
    Path: string;
    Paths: string;
    P: Integer;
begin
    if CurUninstallStep <> usPostUninstall
    then exit;

    Path := ExpandConstant('{app}');

    if not RegQueryStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', Paths)
    then exit;

    P := Pos(';' + Uppercase(Path) + ';', ';' + Uppercase(Paths) + ';');
    if P = 0 then exit;

    Delete(Paths, P - 1, Length(Path) + 1);

    if RegWriteStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', Paths)
    then Log(Format('The [%s] removed from PATH: [%s]', [Path, Paths]))
    else Log(Format('Error while removing the [%s] from PATH: [%s]', [Path, Paths]));
end;

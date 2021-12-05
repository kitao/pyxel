Option Explicit

Function ArgsToString()
    Dim ret
    Dim i
    Dim n

    ret = ""
    n = Wscript.Arguments.Count
    If n > 0 Then
        For i = 1 To n
            If i < n Then
                ret = ret & Wscript.Arguments(i - 1) & " "
            Else
                ret = ret & Wscript.Arguments(i - 1)
            End If
        Next
    End If
    ArgsToString = ret
End Function

Dim batPath
batPath = Replace(WScript.ScriptFullName,".vbs",".exe")

CreateObject("Wscript.Shell").run "cmd /c " & batPath & " " & ArgsToString(), 0

; StratIQ installer colors (dark theme)
; Background and text of the welcome / finish pages and the header bar
!ifdef MUI_BGCOLOR
  !undef MUI_BGCOLOR
!endif
!define MUI_BGCOLOR "0F1923"
!ifdef MUI_TEXTCOLOR
  !undef MUI_TEXTCOLOR
!endif
!define MUI_TEXTCOLOR "ECE8E1"
; The list of files being installed: light text on dark
!ifdef MUI_INSTFILESPAGE_COLORS
  !undef MUI_INSTFILESPAGE_COLORS
!endif
!define MUI_INSTFILESPAGE_COLORS "ECE8E1 0F1923"

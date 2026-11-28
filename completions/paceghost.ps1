# PowerShell completion for paceghost
Register-ArgumentCompleter -Native -CommandName paceghost -ScriptBlock {
  param($wordToComplete, $commandAst, $cursorPosition)
  $subcommands = @('show', 'ghost', 'race', 'season', 'board', 'fieldday', 'ladder', 'rematch', 'meet', 'table', 'reign', 'days', 'forks', 'marks', 'standings', 'traces', 'margins', 'spread', 'middle', 'tails', 'mean', 'median', 'quartiles', 'range', 'midhinge', 'midrange', 'width', '-h', '--help')
  $elements = $commandAst.CommandElements
  $argIndex = 0
  for ($i = 0; $i -lt $elements.Count; $i++) {
    $ext = $elements[$i].Extent
    if ($cursorPosition -le $ext.EndOffset) {
      $argIndex = $i
      break
    }
    $argIndex = $i + 1
  }
  if ($argIndex -eq 1) {
    $subcommands | Where-Object { $_.StartsWith($wordToComplete, [System.StringComparison]::Ordinal) } | ForEach-Object {
      [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterValue', $_)
    }
    return
  }
  if ($argIndex -ge 2 -and $wordToComplete.StartsWith('-')) {
    @('-h', '--help') | Where-Object { $_.StartsWith($wordToComplete, [System.StringComparison]::Ordinal) } | ForEach-Object {
      [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterName', $_)
    }
    return
  }
}

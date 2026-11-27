# PowerShell completion for paceghost
Register-ArgumentCompleter -Native -CommandName paceghost -ScriptBlock {
  param($wordToComplete, $commandAst, $cursorPosition)
  $subcommands = @('show', 'ghost', 'race', 'season', 'board', 'fieldday', 'ladder', 'rematch', 'meet', 'table', 'reign', 'days', 'forks', 'marks', 'standings', 'traces', 'margins', 'spread', 'middle', 'tails', 'mean', 'median', 'quartiles', 'range', 'midhinge', 'midrange', 'width', '-h', '--help')
  $elements = $commandAst.CommandElements
  if ($elements.Count -eq 1 -or ($elements.Count -eq 2 -and $wordToComplete.Length -gt 0)) {
    $subcommands | Where-Object { $_ -like "$wordToComplete*" } | ForEach-Object {
      [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterValue', $_)
    }
    return
  }
  if ($elements.Count -ge 2 -and $wordToComplete.StartsWith('-')) {
    @('-h', '--help') | Where-Object { $_ -like "$wordToComplete*" } | ForEach-Object {
      [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterName', $_)
    }
    return
  }
}

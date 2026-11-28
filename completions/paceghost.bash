# bash completion for paceghost
_paceghost() {
  local cur prev
  COMPREPLY=()
  cur="${COMP_WORDS[COMP_CWORD]}"
  prev="${COMP_WORDS[COMP_CWORD-1]}"
  if [[ $COMP_CWORD -eq 1 ]]; then
    COMPREPLY=( $(compgen -W "show ghost race season board fieldday ladder rematch meet table reign days forks marks standings traces margins spread middle tails mean median quartiles range midhinge midrange width -h --help" -- "$cur") )
    return 0
  fi
  if [[ "$cur" == -* ]]; then
    COMPREPLY=( $(compgen -W "-h --help" -- "$cur") )
    return 0
  fi
  command -v compopt >/dev/null 2>&1 && compopt -o filenames
  local line
  while IFS= read -r line; do
    COMPREPLY+=( "$line" )
  done < <(compgen -f -- "$cur")
  return 0
}
complete -F _paceghost paceghost

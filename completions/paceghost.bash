# bash completion for paceghost
_paceghost() {
  local cur
  COMPREPLY=()
  cur="${COMP_WORDS[COMP_CWORD]}"
  if [[ $COMP_CWORD -eq 1 ]]; then
    COMPREPLY=( $(compgen -W "show ghost race season board fieldday ladder rematch meet table reign days forks marks standings traces margins spread middle tails mean median quartiles range midhinge midrange width -h --help" -- "$cur") )
    return 0
  fi
  if [[ "$cur" == -* ]]; then
    COMPREPLY=( $(compgen -W "-h --help" -- "$cur") )
    return 0
  fi
  return 0
}
complete -o bashdefault -o default -F _paceghost paceghost

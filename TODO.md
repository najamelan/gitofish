# Todo

- abstract out over etc/git2
- thorough evaluation of the error handling.
- simply deny push --force to the deploy branch. See:
  - https://stackoverflow.com/questions/28569039/can-i-disable-force-push-for-certain-branches
  - https://groups.google.com/g/gitolite/c/7CsWB_eOi-I?pli=1

  It seems our configuration already should deny push force, but I recall it happening. We should
  test this well.


- possibly support signed commits, where the server will only accept signed commits.
  server itself could also sign commits, althought this is porbably not useful.

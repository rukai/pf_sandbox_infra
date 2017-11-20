# PF Sandbox Discord Bot

1.  Bot takes commands to add attached fighter.json or stage.json files to the patreon package.
2.  It checks that the user has the required role and that the files are within the file size limit for their role.
3.  It then adds the file to a copy of the package verifies that PF Sandbox will still load and run the new package.
    If the user has already added a stage or fighter then it replaces their current one.
4.  It then replaces the existing package with the new one

*   Periodically check that all fighters and stages belong to a user with the correct roles.

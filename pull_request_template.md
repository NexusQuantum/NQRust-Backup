### Thank you for contributing to the NQRustBackup Project!

#### Please check

- [ ] Short description and the purpose of this PR is present _above this paragraph_
- [ ] Your name is present in the AUTHORS file (optional)

If you have any questions or problems, please give a comment in the PR.

### Helpful documentation and best practices

- [Git Workflow](https://docs.nqrustbackup.org/DeveloperGuide/gitworkflow.html)
- [Automatic Sourcecode Formatting](https://docs.nqrustbackup.org/DeveloperGuide/generaldevel.html#automatic-sourcecode-formatting)
- [Check your commit messages](https://docs.nqrustbackup.org/DeveloperGuide/gitworkflow.html#commits)
- [Boy Scout Rule](https://docs.nqrustbackup.org/DeveloperGuide/generaldevel.html#boy-scout-rule)

### Checklist for the _reviewer_ of the PR (will be processed by the NQRustBackup team)
Make sure you check/merge the PR using `devtools/pr-tool` to have some simple automated checks run and a proper changelog record added.

##### General
- [ ] Is the PR title usable as CHANGELOG entry?
- [ ] Purpose of the PR is understood
- [ ] Commit descriptions are understandable and well formatted
- [ ] Required backport PRs have been created
- [ ] Correct milestone is set

##### Source code quality
- [ ] Source code changes are understandable
- [ ] Variable and function names are meaningful
- [ ] Code comments are correct (logically and spelling)
- [ ] Required documentation changes are present and part of the PR

##### Tests
- [ ] Decision taken that a test is required (if not, then remove this paragraph)
- [ ] The choice of the type of test (unit test or systemtest) is reasonable
- [ ] Testname matches exactly what is being tested
- [ ] On a fail, output of the test leads quickly to the origin of the fault

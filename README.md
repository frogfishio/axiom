# axiom

This repository contains the Axiom family of tools, including SDA, a small language and command-line tool for reading, checking, and reshaping structured data.

## Your First 3 SDA Commands

If you want to try SDA immediately, start with these:

```sh
sda check -f SDA/examples/getting_started/person_name.sda
sda fmt -f SDA/examples/getting_started/person_name.sda --check
sda eval -f SDA/examples/getting_started/person_name.sda -i SDA/examples/getting_started/person.json
```

These three commands answer three different questions:

1. is the SDA source valid?
2. is it formatted correctly?
3. what happens when I run it on real data?

Start here if you want the approachable guide rather than the formal specification:

- [SDA User Manual](SDA/USER_MANUAL.md)
- [SDA Cheat Sheet](SDA/CHEATSHEET.md)
- [SDA for jq Users](SDA/FOR_JQ_USERS.md)
- [SDA Scholarly Introduction](SDA/INTRODUCTION.html)
- [Commands Overview](COMMANDS.md)

If you need the formal contract instead of the beginner guide:

- [SDA Specification](SDA/SDA_SPEC.md)

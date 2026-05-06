# DomainModelMaker
A web based application for quickly making domain model diagrams.

## Usage

In order to create a relation between two entities, you write it with the following format:
```
Jack-Jill
```
<img width="220" height="120" alt="image" src="https://github.com/user-attachments/assets/51685edb-707f-4b0b-8885-5334fca48fba" />


You can also label relationships and show directionality:
```
Jack<-knows->Jill
```
<img width="220" height="120" alt="image" src="https://github.com/user-attachments/assets/f3dc8375-43a4-4ddd-9b83-fb7329a66017" />

You can show multiplicity by adding a number:
```
Jack-fetches-1>Pail of water
Peter Piper-picked-1..>Pickled Peppers
```
<img width="220" height="120" alt="image" src="https://github.com/user-attachments/assets/65c508a5-a4d1-4e76-8f45-f02e603be8ea" />

Using this syntax, you can create domain models (or model other relationships)

<img width="320" height="240" alt="image" src="https://github.com/user-attachments/assets/cc390bb8-a6c7-4e1a-9c5f-5cc1503d324b" />

There are some other tools you can use as well.

Writing `pin EntityName: <x-coordinate> <y-coordinate>` will pin an entity to one spot. (Keep in mind, negative coordinates are written with a tilde `~` rather than a dash `-`.)

If an entity name contains a dash, number, or other symbol, you can escape it with a backslash (e.g. `Vec\2`), or you can surround the entity name in quotes (e.g. `"First-Person Camera"`). Double quotes (`"`), single quotes (`'`), and ticks (``` ` ```) are supported.

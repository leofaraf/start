## Roadmap

### Operations

1. Find
- [X] Basic execution
- [ ] Handle filtering, limiting, etc.
- [ ] Indexing (B+tree)
- [ ] Mongo-style cursor 

2. Insert
- [X] Simple one-doc insert
- [ ] Ids
- [ ] Indexing (B+tree)

3. Update
- [ ] Simple bulk update by filter

4. Delete
- [ ] Delete operation by filters

### ACID

Now, impliment AC__ (atomicity, consistency, isolation, durability).

1. Atomicity
- [X] Basic functional for all operations (RecoveryUnit)
- [X] Session catalog (TransactionParticipant -> txnNumber, recovery unit (changes) )
- [X] Multi-document transactions (based on session catalog)
- [X] Refactor Recovery Unit
- [X] Fully “All or Nothing” atomocity

2. Consistency
- [X] Develop new: Inconsistent because of "collection catalog"
- [ ] Validation hooks or constraints during commit

3. Isolation
- [ ] Locks

4. Durability
- [ ] WAL (Write Ahead Log)
- [ ] Crash recovery on startup

### Rest

1. More efficent Space managment system
- [ ] Trash (_system_trash)
- [ ] Storage pages like in mongo

2. Redesign filters? (mb include limits etc. inside)

3. SQL (StartDB Query Language) parsing
- [ ] Define SQL Grammar

4. Stabilization
- [ ] Redesign ensure capacity
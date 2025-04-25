## Roadmap

### Operations

1. Find
- [X] Basic execution
- [X] Handle filtering, limiting, etc.
- [ ] Indexing (B+tree)
- [ ] Redesign plan executor
- [ ] Mongo-style cursor 

2. Insert
- [X] Simple one-doc insert
- [ ] Ids
- [ ] Indexing (B+tree)

3. Update
- [X] Simple bulk update by filter

4. Delete
- [X] Delete operation by filters
- [ ] Create _system_trash

### ACID

Now, impliment AC__ (atomicity, consistency, isolation, durability).

1. Atomicity
- [X] Basic functional for all operations (RecoveryUnit)
- [X] Session catalog (TransactionParticipant -> txnNumber, recovery unit (changes) )
- [X] Multi-document transactions (based on session catalog)
- [X] Refactor Recovery Unit
- [X] Commit and rollback should end current transaction
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

2. SQL (StartDB Query Language) parsing
- [X] Define SQL Grammar
- [ ] Create AST (Abstract Syntax Tree)
- [ ] Implement the Parser
- [ ] Query Interpreter

3. Stabilization
- [ ] Redesign ensure capacity
- [X] User-friendly insert, find
- [ ] Benchmark how much memory I need
- [ ] Redesign some function argument to BSON
- [X] Errors redesign
- [ ] Create database error master type 
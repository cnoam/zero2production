I see that the db component is running (looking at DO portal), but cannot connect to it, using the connection string they give.
I DID turn of "trusted sources"


When posting to /subsciptions , server returns code 500


2023-11-12: I want to get the DO dbase working before moving to the next chapter. Looking at a reference https://github.com/aboqasem/zero2prod/blob/e03ea408aa16c21b181041a104e21b692b3d7217/deploy/digitalocean/spec.yml#L4
for a delta

---------------
build problem:
`cargo build` hangs -> timeout connecting to DB. from command line in my working dir.
in the reference repo, it complains on missing clang (correctly).

I ran `cargo cache -a` and installed clang. no change.

in pycharm it compiles and run.


in the reference repo, `apt install clang lld`. 
Now the reference compiles ok.

=========
|| GOAL: deploy to DO
=========
2023-11-14 : SUCCSESS!
using the updated book and source code of chapter 5


2023-12-4: in page 412, need to copy the code from the book, since I don't want to write it myself.

2023-12-23: code from 10.6.3.1 seems to be working.
Now need to check it runs, and continue in 10.6.3.2


2024-01-07: arrived to 10.7.5.3 in page 491 

2024-08-11 13:40   The above comment is on the file zero2prod_with_cover_20230921.pdf
Moving from pycharm to rustrover 2024.2

To run the code(commit ec4f6fc0cc) in my local shell:
```
export SQLX_OFFLINE=1
./scripts/init_db.sh
./scripts/init_redis.sh
cargo run
```

and then the `localhost:8000` is ready.





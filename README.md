## Discord Rust Bot
A small discord bot written in Rust using the Serenity library. It will keep track of all of the
messages sent and add XP for each member for each guild. It will also create a private `voice-only` channel
that the bot will automatically place you in when you join a voice channel, as to keep any conversation-related message
confined to the `voice-only` channel.

### How to run compile & run

Step 1 - Clone the repository
----

Clone the repository and then change directory with this command

```
git clone https://github.com/batfolx/rust_discord_bot.git && cd rust_discord_bot
```

Step 2 - Create `script.sh`
----

Create a shell script called `script.sh` with this command

```
touch script.sh
```


Step 3 - Put discord token in `script.sh`
----

inside of `script.sh`, place your discord token

```
cargo run <YOUR-DISCORD-TOKEN-HERE>
```

Step 4 - Run the script
----

Make sure you have the proper permissions on `script.sh` by executing the command

```
chmod +x script.sh
```

and then run the script

```
./script.sh
```





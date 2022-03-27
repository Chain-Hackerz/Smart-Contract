We are building a speed runner app that incrementally sends data to a smart contract which acts as a speed verifying tool.

It gets user input which is in the form of key presses during a game and we incrementally send data to the block chain to be verified.

ExecuteMessage when given a vector of inputs keys pressed for the game interval, just updates the vector of key presses saved to the blockchain.

It can alternatively also be called to check for manipulation signs in the input file of key presses just by looking if the time stamps are in sequence. By looking at the difference in time stamps of each consequent key presses, we were able to look at whether or not the key presses are in sequence.

If they are not, it sends a contract error saying that the verification was rejected.

If they are in sequence, we return a response containing the mean of the stats to the user. 

Ideally, we wanted to look at the distribution of the time stamps. If the mean difference in time stamps was below the average for a particular game, it would be a clear sign of manipulation. Due to time constraints, we could not implement that feature.

We ideally wanted to extract a the frames with time stamps for a streaming of a video game and check if the time stamps are in sequence but unfortunately we could not find a working library for screen capture with the time stamps.

The player can finally queries the blockchain to get back his input file while is immutable once it is saved on the blockchain and cannot be tampered with.


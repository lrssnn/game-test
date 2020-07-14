# Baseball Simulator
This is a baseball simulator of unknown seriousness, to play around with game programming in rust.

## Design sketch
I think we should start by thinking about a pitch:
1. The pitcher chooses a pitch - a particular pitcher should have a certain set of pitches, and a comfort level/frequency for each one. Maybe a pitcher has pitch sequences? Or the sequence is generated from some primitive stored in both the pitcher and the catcher
2. The pitcher chooses a location - this is encompassed in the same decision making as above
3. Some sort of accuracy + power calculation - pitcher skill, pitch difficulty, stamina level
4. The pitch is now travelling to the hitter. There needs to be some calculation of pitcher vision - the faster a pitch is the harder it is to hit, but the more similar it is to the other pitches the hitter has seen, the easier it gets to adjust to it. Similar for pitch movement, similar pitches in a row become more visible. Hitters should probably learn from repeated ABs against the same pitcher, maybe also from watching other hitters against them.
5. Hitter decides when to swing (or not). I'd probably start with high-mid-low, out-mid-in, and a timing (based on guessed pitch speed). All right, cracking hit, above the pitch is a popup, below is a tip, off horizontally is likely a foul...
6. If the ball is hit, we need to determine the trajectory. Probably ambitious to do this physically - look at the above factors and determine a direction and a launch angle. From this we get landing spot and time taken.
7. Determine fielder actions. Simple for nobody on, the correct actions for multiple people on is complicated...


## Program Structure
After some experimenting, I think the program should be structered as so:

I see the UI being multiple panels, e.g. A panel which displays the pictch in the strike zone, a panel which shows the fielders and runners on the field, a panel with text output (play-by-play details etc).
We should have a "Game State" struct which contains everything about the game. I think this class should also hold the logic for the gameplay. Then multiple subscreens can just take that struct and render whatever part they care about.
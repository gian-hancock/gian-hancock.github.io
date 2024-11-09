---
title: "GBA Jam 2024"
date: 2024-11-09
author: Gian Hancock
toc: true
githubDiscussionUrl: 'https://github.com/gian-hancock/gian-hancock.github.io/discussions/3'
draft: false
# Videos and screenshots taken from Mesen emulator
# To get small but sharp videos without artifacts, I recorded using the raw option in Mesen, then compressed, upscaled and converted to webm using ffmpeg using the following command:
# ffmpeg -i .\cafe-closed.avi -c:v libvpx -vf scale=2*in_w:2*in_h:flags=neighbor -crf 10 -b:v 1M -c:a libopus cafe-closed-2crf.webm
---

## 1. GBA Jam 2024

This year [Reggie](https://eragnarok.itch.io/) and I participated in [GBA Jam 2024](https://itch.io/jam/gbajam24), a 3 month game jam for the GameBoy Advance. As this was my first project for the GBA, and my first time using C++, my personal goal was simply to make something of high enough quality to submit. 

Overall, we both had a great time and we're already excited for the next one. This post is a summary and reflection on my experience.

## 2. The Game & Results
Our entry [Detective Monroe: Murder at Sea](https://eragnarok.itch.io/detective-monroe-murder-at-sea) was a point and click style murder mystery. While not groundbreaking in terms of design or mechanics, we tried to deliver a complete experience that can be played from beginning to end. 

<div style="width: 90vw; position: relative; left: 50%; right: 50%; margin-left: -45vw; display: flex; flex-wrap: wrap; justify-content: center; gap: 10px;">
  {{<figure src="cafe.png" width="600px" class="image-rendering-pixelated" alt="Gameplay image: The player standing at the cafe in the ship.">}}
  {{<figure src="announcement.png" width="600px" class="image-rendering-pixelated" alt="Gameplay image: An announcement being made to the ships passengers.">}}
  {{<figure src="puzzle.png" width="600px" class="image-rendering-pixelated" alt="Gamplay image: An in-game puzzle.">}}
  {{<figure src="lockbox.png" width="600px" class="image-rendering-pixelated" alt="Gameplay image: A lockbox with a 4 digit code.">}}
</div>

We managed to put in a solid entry, landing in a crazy 5-way tie for [4th place](https://itch.io/jam/gbajam24/results) out of 52 submissions; we also placed 2nd in the [open source category](https://gbadev.net/gbajam24.html#open-source-bonus).

{{<figure src="results.png" caption="Games were ranked in multiple categories, but only Overall was used for raking and prizes" alt="GameJam ratings: 4th in Overall, 5th in Polish, 7th in Originality, 15th in Technical, 16th in Graphics, 32nd in Audio.">}}

## 3. Thoughts and Reflections

### 3.1. Teamwork
For this project—as with most of our projects—Reggie had the idea & vision, and I was happy to focus on the technical aspects. In general Reggie was a generalist working on all aspects of the game, while I primarily contributed code. This has always been an enjoyable way of working for us, and everything went quite smoothly.

Since this was an evenings and spare time sort of thing, we mostly worked asynchronously. Keeping our chat populated with updates on what we were working on was important, it was a boon for motivation, and it helped make sure I was always working on the "right" thing.

{{<figure src="ship-outline.png" width="600px" caption="Quality content from our Discord chat" alt="A sample screenshot from our Discord chat. Reggie posts a rough image of the layout of the ship.">}}

There were definitely some occasions where it would have been nice to be working simultaneously, such as when Reggie started working with the dialogue system that I had created, which wasn't exactly well documented. In a similar vein, there were some opportunities for me to add simple features that would have cut down on the boilerplate Reggie had to write, unfortunately I didn't realise this in time.

We both took a day off work on the final Friday of the jam; in hindsight one Friday a month together would have been a big productivity boost.

### 3.2. Pacing
Out of curiosity I browsed through our Git and chat history to put together a list of milestones. I also included how far along the 3 month jam each one occurred:

- **33% - Game plot:** Plot of the game is mostly in its final form, but dialogue is not actually written.
- **33% - "Proper" coding started:** We move from toying with GBA dev to actually making progress on the game.
- **50% - Dialogue system:** System is usable, but we haven't actually put any content into the game yet.
- **70% - Gameplay programming:** We're starting to add cutscenes, puzzles, dialogue.
- **93% - Inventory system:** pick up items, combine and use them.
- **97% - Playthrough:** We try our first end to end playthrough of the game. Many bugs were discovered, some of them game breaking.
- **1hr 20 mins before submission - Ambient sound and music:** We put in some ambient sound and music in some key areas, but most areas of the game remain quiet.
- **1hr before submission - Title screen:** Title screen with transition to initial cutscene.
- **30 mins before submission - Final bug fixes:** The last two bugs we fixed ranged from unimportant to critical:
    - Elevator "ding" sound playing when it isn't supposed to.
    - Final puzzle not triggering ending in all cases.

Almost all of the development occurred in the final 2/3 of the jam, and maybe 50% of it was done in the last 1/4. That's not ideal, but I'm generally happy with this. We definitely built up motivation and momentum as the project went on, which wasn't such a bad thing.

### 3.3. Dialogue System
One particularly load bearing part of our game is the dialogue system. The game is dialogue heavy and the gameplay and dialogue are intertwined.

From a technical perspective the system is quite simple. Dialogue "Nodes" are the core of the system. Each Node contains some text which is displayed on the screen and a "next" value, indicating the next snippet of dialogue (if any). On top of this are a few simple features to make the dialogue more interactive:
- **Choices:** Allows for branching conversation.
- **Predicates:** Allows a choice to be shown only if some "predicate" is true.
- **Actions:** Allows dialogue to trigger in-game events.
- **Auto-choices:** Allows dialogue to branch based on in-game conditions. We implemented this on top of choices by allowing the game to pick a choice automatically without ever displaying it to the player.

<div style="width: 90vw; position: relative; left: 50%; right: 50%; margin-left: -45vw; display: flex; flex-wrap: wrap; justify-content: center; gap: 10px;">
  {{<figure 
        src="reception-dialogue-choice.png" 
        width="500px" 
        caption="A dialogue choice presented to the player" 
        class="image-rendering-pixelated"
        alt="An image demonstrating the player being prompted to make a dialogue choice."
  >}}
  <figure>
    <video controls muted width="500px" alt="A video demonstrating available dialogue choices changing as the game progresses.">
        <source src="./receptionist-room-card-clip.webm" type="video/webm">
        Your browser does not support the video tag.
    </video>
    <figcaption>
    The bottom choice changes from "Room Card" to "My Room" based on predicates
    </figcaption>
  </figure>
  <figure>
    <video controls muted width="500px" alt="A video demonstrating an action triggered by dialogue, an NPC runs off screen.">
        <source src="./laxative-clip.webm" type="video/webm">
        Your browser does not support the video tag.
    </video>
    <figcaption>
    NPC movement triggered by an Action
    </figcaption>
  </figure>
    <figure>
    <video controls muted width="500px" alt="A video demonstrating an auto-choice causing the dialogue to branch depending on in-game circumstances.">
        <source src="./shop-clip.webm" type="video/webm">
        Your browser does not support the video tag.
    </video>
    <figcaption>
    An auto-choice selects the "$10" branch the first time, and the "$4" one afterwards
    </figcaption>
  </figure>
</div>


#### 3.3.1. Unexpected Uses
The dialogue system ended up being flexible enough to (mis)use it for basically all interactions:

- **Opening a door?** More like having a conversation with it. It might trigger an action that takes you to the next room, or it might tell you it's locked and to go away.
- **Using an elevator?** Basically the same as a door, except it asks you which floor you'd like first.
- **Picking up an item?** You guessed it, you talk to the item, it says something like `* Rope Obtained *` and triggers an Action which adds the item to your inventory.
- **Cutscenes?** These are just long conversations between characters, with Actions sprinkled in to move the camera and the NPCs.

We used the dialogue system everywhere we could to save time, even some of the puzzles are powered by a mixture of dialogue and puzzle specific code.

#### 3.3.2. Authoring Dialogue
Initially we hardcoded the dialogue Node data in a const array in the C++ source. This was too unwieldy even for a jam—it was demonstrably error-prone and caused hard to detect errors[^hard-coded-node-bug]. We ended up code generating this C++ from a YAML document. This was a happy medium between hard coding the Nodes and a full DSL[^ink-yarn-spinner].

The code generation had some quality-of-life features such as allowing branching conversation to be represented with nesting branches in the YAML, and inferred defaults for common cases. I went so some effort to provide human readable error messages if the provided YAML was incorrect (valid YAML but semantically incorrect).

Below is an interaction with the elevator and the YAML that powers it:

<div style="width: 90vw; position: relative; left: 50%; right: 50%; margin-left: -45vw; display: flex; flex-wrap: wrap; justify-content: center; gap: 10px;">
    <figure>
        <video controls loop muted width="500px" alt="A video showing the player using the elevator, the elevator denies them access to the 4th floor.">
            <source src="./elevator.webm" type="video/webm">
            Your browser does not support the video tag.
        </video>
    </figure>
</div>

```yaml
- entry: Elevator
  text: Which floor?
- options:
    a:
      - text: "1st: Cabins"
        action: GoTo1stFloor
    b:
      - text: "2nd: Bar"
        action: GoTo2ndFloor
    c:
      - text: "3rd: Cafe"
        action: GoTo3rdFloor
    d:
      - text: "4th: Staff only"
      - options:
          a:
            - predicate: DoesntHaveStaffCard
              text: null
            - text: Error - Access Card Required
          b:
            - predicate: HasStaffCard
              action: GoTo4thFloor
              text: null
  next: end
```

We see an example of branches being nested inside each other in the `d` branch: `"4th: Staff only"`. Also note that if an option has no text associated with it (`text: null`) then it's an auto-choice and is selected by the game as long as its predicate passes. This is how option `d.a` or `d.b` is chosen automatically depending on whether the player has the access card.

#### 3.3.3. Linking Dialogue to Gameplay
In the above YAML snippet, we've seen actions (e.g. `action: GoTo3rdFloor`) and predicates (e.g. `predicate: HasStaffCard`). How are they linked to gameplay?

For actions we used a basic approach, an enum was generated with every possible action: 

```c++
enum class DlgAction {
    GoTo1stFloor
    GoTo2ndFloor
    // and many more...
};
```

A single global handler is called every time an action occurs:

```c++
void SceneMain::HandleDlgAction(DlgAction a) {
  BN_LOG("HandleDlgAction(", kDlgActionNames[(int)a], ")");
  switch (a) {
    case DlgAction::None: {
    } break;
    case DlgAction::GoTo3rdFloor: {
      // etc...
      next_scene_ = NextScene{.scene_type = SceneType::SCENE_3RD_FLOOR, .room_info = &room_data};
    } break;

    // Many more cases omitted...
  }
```

It's crude, but it was good enough for our short-lived project.

Conditional branches are handled via predicates. In an effort to keep the system as minimal as possible, the dialogue YAML allows predicates to be named, but doesn't provide any syntax for specifying how they are evaluated. Instead the conditional logic is authored outside of the YAML, in a similar way to actions[^retained-state-ink-yarn]:

```c++
bool SceneMain::CheckDlgPredicate(DlgPredicate predicate) {
  bool result;
  switch (predicate) {
    case DlgPredicate::None:
      result = true;
      break;
    case DlgPredicate::HasLeadTestedCoat: {
      result = globals.IsInInventory(InventoryItem::CoatLeadTested);
    } break;

    // Many more cases omitted...
  }

  // logging
  return result
```

Although unrelated to dialogue, the item "combine" and "use" actions follow a similar pattern, global handlers which receive enum parameters.

### 3.4. Completeness
We submitted a complete game experience which can be played from beginning to end, and I think this served us well. Looking at our ratings (see [Game & Results](#2-the-game--results) section) we see that we did best in the "Overall" category, which was the only category considered for ranking the games for prizes. We managed this despite much lower scores in "Technical", "Graphics" and "Audio". I feel this is largely due to the completeness of our entry.

In addition to the game, we provided a written [walkthrough](https://docs.google.com/document/d/1nm3FMgB4hf-deIHZv5DgcbHVVbcXWF7k1PJE0deie9I). We wanted to make sure everyone could see all the game content, even if they got stuck. Ideally the puzzles would make sense and would be rewarding enough for players to solve on their own, but I think this was a pragmatic choice. At least one judge used the walkthrough, so it was worth it in the end.

> The walkthrough manual is a great touch, thanks! I got stuck with the hook.
> 
> — Judge feedback

### 3.5. Polish
We were happy with the level of polish upon submission—"Polish" was our second-best criteria after "Overall"—but there was still plenty of room for improvement.

#### 3.5.1. Dialogue
We essentially rolled with our initial dialogue, with a few tweaks here and there. I personally aimed to do a full pass before submission, but we didn't have time in the end. We realised that it was too verbose in places, so I did a hack job of cutting out as much as I could without changing the plot or taking away the from character of the game.

More "colour" dialogue with random NPCs in the game would have been nice.

#### 3.5.2. Bugs
There was only 1 bug that we were aware of: missing collision on one wall allowing you to clip into another room. We simply ran out of time to fix it. Fortunately, this didn't allow any sequence breaks and was mainly a polish issue. This was reported by 2 players, but gladly no other bugs were mentioned.

#### 3.5.3 Music & Sound
I was surprised how much feedback—both positive and negative—we received about the sound in our game.

- > It's a shame that there's no music during most of the game.

- > The audio is missing in some parts and it would be great to have a bit of ambient music.

- > I also liked the small details such as the lounge music and coffee shop ambience getting louder the closer you were to the band/counter.

- > Wish it had some music though, it's a key part of mystery films!

We totally underestimated the impact of sound. Just an hour of time adding royalty free music would have made quite a difference.

#### 3.5.4. Graphics
Graphics are time intensive, and a whole skill of their own, so "make graphics better" is not particularly insightful. But it is worth noting that small things can have a disproportionate impact. The title screen uses a wide font on the "start game"/"load save" buttons, it doesn't fit well but I just couldn't be bothered changing it. The title screen is the first impression, and this font was indeed the subject of someone's very first reaction to the game. I should keep this in mind as I tend to underestimate the impact of little things like this.

#### 3.5.5. Testing
While I wouldn't say we tested our game extensively, we did multiple full run throughs before submitting. We found and fixed a surprising number of issues as a direct result of this. It's always easy to neglect properly playtesting in game jams, but our experience here really shows how important it is. One notable change we made is doubling the walking speed of the main character, it wasn't until I was playing the game end to end that I realised we needed to do this.

>...and traversing the ship was quick enough that I never got too frustrated even in the moments where I was stuck.
> 
> — Itch.io comment

### 3.6. Motivation

Overall, it was a very enjoyable project to work on and we were both highly motivated to see it through. There are a few primary factors which led to this:

- **Technologies:** This was my first project in C++, and my first GBA project. I felt like I was constantly learning, but the learning curve wasn't so steep that we weren't making progress. I fell back to Rust for writing the dialogue code generator, which added some familiarity and variety to the project.
- **Team:** Working in a small highly aligned teams has always been enjoyable for me. It's nice working people who share your enthusiasm for a project, while still benefiting from minimal coordination overhead.
- **Deadline:** Having a clear deadline definitely helped push me to keep the pace up. It also forced us to set a reasonable scope early on.

## 4. Wrap Up
Overall GBA Jam was a big success, with more submissions than ever. Thanks to the [community](https://gbadev.net/) who organised and judged this jam. It ran smoothly and the judges left quality written feedback on the entries which we really appreciated.

I'd also like to thank the [sponsors and donors](https://opencollective.com/gbadev/projects/gbajam24#section-contributors). In addition to cash prizes, there were GBA related physical prizes too, which I thought was pretty cool.

And of course, thanks to the other participants, the general sentiment is that the quality of submissions was top notch this year, so well done everyone! In a serendipitous discovery, we also got in touch with one of the participants who happens to live in our area. What are the chances‽

Finally, a shout out to [Butano](https://github.com/GValiente/butano). Butano engine made it possible for two C++ and GBA noobs to see our vision through.

[^hard-coded-node-bug]: 
    A notable example: the hand-coded initialiser of one particular Node was missing a comma: 
    ```c++
    DialogNodeStandard{"Nope, scram!"  -1}
    // Should be a comma here: ------^
    ```
    The initializer is supposed to set the Node text and set the next Node index to -1 (Which means the dialogue stops after this Node). The missing comma here was not a compiler error, instead it caused the initialiser list to be interpreted as a single element: a pointer to the byte before the `char*` value `"Nope, scram!"`. The second element was default initialised to 0. This caused the Node to display no text and then jump to an unrelated piece of dialogue afterwards.

[^ink-yarn-spinner]:
    I did experiment with [ink](https://www.inklestudios.com/ink/) and [Yarn Spinner](https://www.yarnspinner.dev/) but I didn't feel like I had the time to integrate them with our game.

[^retained-state-ink-yarn]:
    [ink](https://www.inklestudios.com/ink/) and [Yarn Spinner](https://www.yarnspinner.dev/) *do* provide syntax for conditional logic, allowing dialogue related logic to be handled directly by the dialogue system. This convenient for authoring dialogue, but it does require expression syntax to be added to the language. Additionally, it requires the dialogue system to retain state that may be needed for expression evaluation, such as how many times each Node has been visited, and even author defined variables.

    One benefit of avoiding all of this—beyond simply having less to implement—is that the dialogue system has no state to worry about when it comes to game loading/saving.
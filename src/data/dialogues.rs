//! Dialogue trees for each dateable fish, built with sable-dialogue.

use sable_dialogue::prelude::*;
use sable_dialogue::dialogue::DialogueBuilder;

use crate::data::FishId;

/// Build the dialogue tree for a specific fish.
pub fn build_dialogue(fish_id: FishId) -> DialogueTree {
    match fish_id {
        FishId::Bubbles => build_bubbles_dialogue(),
        FishId::Marina => build_marina_dialogue(),
        FishId::Gill => build_gill_dialogue(),
    }
}

fn text_node(id: &str, speaker: &str, text: &str, next: &str) -> DialogueNode {
    DialogueNode::Text {
        id: id.into(),
        speaker: Some(speaker.into()),
        emotion: None,
        text: text.into(),
        text_key: None,
        next_node: Some(next.into()),
        actions: Vec::new(),
        voice_clip: None,
    }
}

fn build_bubbles_dialogue() -> DialogueTree {
    DialogueBuilder::new("start")
        .title("Date with Bubbles")
        .speaker(Speaker::new("bubbles", "Bubbles"))
        .speaker(Speaker::new("player", "You"))
        .node(text_node(
            "start",
            "bubbles",
            "Hey hey hey! Thanks for bringing me to the Coral Cafe! I LOVE this place!",
            "q1",
        ))
        .node(DialogueNode::Choice {
            id: "q1".into(),
            prompt: Some("Bubbles is bouncing excitedly. What do you say?".into()),
            speaker: None,
            choices: vec![
                Choice::new("Your enthusiasm is contagious! What's good here?", "q1_good")
                    .sets("affection", 3_i32),
                Choice::new("Calm down, it's just a cafe.", "q1_neutral"),
                Choice::new("I mostly came for the free breadsticks.", "q1_funny")
                    .sets("affection", 2_i32),
            ],
        })
        .node(text_node(
            "q1_good",
            "bubbles",
            "Oh em gee, EVERYTHING! But the kelp smoothie is to DIE for. Get it? Die? Like... fish don't actually die from kelp... okay that was dark.",
            "q2",
        ))
        .node(text_node(
            "q1_neutral",
            "bubbles",
            "Oh... yeah, I guess you're right. *fidgets* I just get excited, you know?",
            "q2",
        ))
        .node(text_node(
            "q1_funny",
            "bubbles",
            "HA! You're funny! I like funny. The breadsticks here are actually shaped like little seahorses!",
            "q2",
        ))
        .node(DialogueNode::Choice {
            id: "q2".into(),
            prompt: Some("Bubbles looks at you expectantly.".into()),
            speaker: None,
            choices: vec![
                Choice::new("Tell me about yourself, Bubbles.", "q2_deep")
                    .sets("affection", 4_i32),
                Choice::new("So... do you have any hobbies?", "q2_hobby")
                    .sets("affection", 2_i32),
                Choice::new("*stare at menu in silence*", "q2_silence"),
            ],
        })
        .node(text_node(
            "q2_deep",
            "bubbles",
            "Aww, you want to know ME? Well, I'm a clownfish! I live in an anemone with my family. I love making others laugh because... honestly? The ocean can be scary sometimes. Laughter helps.",
            "q3",
        ))
        .node(text_node(
            "q2_hobby",
            "bubbles",
            "I collect shiny things! Bottle caps, coins, once I found a whole spoon! I keep them in my anemone. My roommate hates it.",
            "q3",
        ))
        .node(text_node(
            "q2_silence",
            "bubbles",
            "... ... ... Soooo this is awkward! I'll just keep talking then! Did you know clownfish can change gender? Nature is WILD!",
            "q3",
        ))
        .node(DialogueNode::Choice {
            id: "q3".into(),
            prompt: Some("The date is going well. Final moment...".into()),
            speaker: None,
            choices: vec![
                Choice::new("I had a really great time tonight, Bubbles.", "ending_good")
                    .sets("affection", 5_i32),
                Choice::new("You're the funniest fish I've ever met.", "ending_great")
                    .sets("affection", 4_i32),
                Choice::new("Well, this was... something.", "ending_meh")
                    .sets("affection", 1_i32),
            ],
        })
        .node(text_node(
            "ending_good",
            "bubbles",
            "Me too! Can we do this again? I know this great place where the bioluminescent plankton glow at night!",
            "end",
        ))
        .node(text_node(
            "ending_great",
            "bubbles",
            "*turns bright orange* Stop it, you're making me blush! ...Wait, I'm ALWAYS orange. BUT THE POINT STANDS!",
            "end",
        ))
        .node(text_node(
            "ending_meh",
            "bubbles",
            "Oh... okay! Well, the offer stands if you ever want to hang out! No pressure! *nervous laugh*",
            "end",
        ))
        .node(DialogueNode::end("end"))
        .build_unchecked()
}

fn build_marina_dialogue() -> DialogueTree {
    DialogueBuilder::new("start")
        .title("Date with Marina")
        .speaker(Speaker::new("marina", "Marina"))
        .speaker(Speaker::new("player", "You"))
        .node(text_node(
            "start",
            "marina",
            "Hmph. The Moonlit Reef. Acceptable choice. I've seen better, but... the view isn't terrible tonight.",
            "q1",
        ))
        .node(DialogueNode::Choice {
            id: "q1".into(),
            prompt: Some("Marina gazes at the moonlit water with cool detachment.".into()),
            speaker: None,
            choices: vec![
                Choice::new("I picked it because the moonlight matches your silver scales.", "q1_flirt")
                    .sets("affection", 4_i32),
                Choice::new("I hear you're the fastest fish in these waters.", "q1_compete")
                    .sets("affection", 3_i32),
                Choice::new("What, too fancy for you?", "q1_snarky")
                    .sets("affection", 1_i32),
            ],
        })
        .node(text_node(
            "q1_flirt",
            "marina",
            "*pauses* ...That was... smoother than I expected. Don't think flattery will make me go easy on you, though.",
            "q2",
        ))
        .node(text_node(
            "q1_compete",
            "marina",
            "The fastest? Please. I'm the fastest in the ENTIRE eastern reef system. I clocked 60 knots last Tuesday. Care to race?",
            "q2",
        ))
        .node(text_node(
            "q1_snarky",
            "marina",
            "Careful with that attitude. I have a sword on my face and I'm not afraid to use it. ...I'm kidding. Mostly.",
            "q2",
        ))
        .node(DialogueNode::Choice {
            id: "q2".into(),
            prompt: Some("Marina seems to be warming up slightly.".into()),
            speaker: None,
            choices: vec![
                Choice::new("What drives you to be the best at everything?", "q2_deep")
                    .sets("affection", 4_i32),
                Choice::new("Want to have a race right now?", "q2_race")
                    .sets("affection", 3_i32),
                Choice::new("You seem really intense.", "q2_blunt")
                    .sets("affection", 1_i32),
            ],
        })
        .node(text_node(
            "q2_deep",
            "marina",
            "...Nobody asks me that. They just see the speed, the sword, the attitude. But... I push myself because stopping means being forgotten. And I refuse to be forgotten.",
            "q3",
        ))
        .node(text_node(
            "q2_race",
            "marina",
            "Ha! Now you're speaking my language! Three laps around that coral formation. Loser buys dinner. Ready? ...Actually, let's finish our date first. Then I'll destroy you.",
            "q3",
        ))
        .node(text_node(
            "q2_blunt",
            "marina",
            "Intense is how legends are made. But... maybe tonight I can dial it down. Just a notch. For you.",
            "q3",
        ))
        .node(DialogueNode::Choice {
            id: "q3".into(),
            prompt: Some("The moonlight glitters across the reef.".into()),
            speaker: None,
            choices: vec![
                Choice::new("You don't have to perform for me, Marina. I like who you really are.", "ending_good")
                    .sets("affection", 5_i32),
                Choice::new("Next time, I choose the spot. And we're racing.", "ending_great")
                    .sets("affection", 4_i32),
                Choice::new("Thanks for the evening. It was... educational.", "ending_meh")
                    .sets("affection", 1_i32),
            ],
        })
        .node(text_node(
            "ending_good",
            "marina",
            "*long pause* ...You might be the first person to say that to me. *looks away* ...Same time next week?",
            "end",
        ))
        .node(text_node(
            "ending_great",
            "marina",
            "Deal. But I'm warning you - I don't lose. *small genuine smile* This was... not terrible. At all.",
            "end",
        ))
        .node(text_node(
            "ending_meh",
            "marina",
            "Educational. Right. Well... good night then. *swims away quickly*",
            "end",
        ))
        .node(DialogueNode::end("end"))
        .build_unchecked()
}

fn build_gill_dialogue() -> DialogueTree {
    DialogueBuilder::new("start")
        .title("Date with Gill")
        .speaker(Speaker::new("gill", "Gill"))
        .speaker(Speaker::new("player", "You"))
        .node(text_node(
            "start",
            "gill",
            "Oh! H-hi! I didn't think you'd actually show up... *puffs up slightly* S-sorry, that happens when I'm nervous...",
            "q1",
        ))
        .node(DialogueNode::Choice {
            id: "q1".into(),
            prompt: Some("Gill is visibly nervous, slightly puffed up.".into()),
            speaker: None,
            choices: vec![
                Choice::new("It's okay, take your time. I'm in no rush.", "q1_kind")
                    .sets("affection", 4_i32),
                Choice::new("You look adorable when you puff up like that!", "q1_cute")
                    .sets("affection", 3_i32),
                Choice::new("Why would I not show up?", "q1_confused")
                    .sets("affection", 2_i32),
            ],
        })
        .node(text_node(
            "q1_kind",
            "gill",
            "*slowly deflates* ...Thank you. That's... really nice of you. Most fish get scared when I puff up. This sunken ship is actually my favorite place.",
            "q2",
        ))
        .node(text_node(
            "q1_cute",
            "gill",
            "*PUFFS UP MORE* D-don't say that! *tiny voice* ...but thank you... nobody's ever called it adorable before...",
            "q2",
        ))
        .node(text_node(
            "q1_confused",
            "gill",
            "I dunno... I'm not exactly the most exciting fish in the sea. I'm small, I puff up weird, and I mostly just... think about stuff.",
            "q2",
        ))
        .node(DialogueNode::Choice {
            id: "q2".into(),
            prompt: Some("Gill has calmed down and seems more comfortable.".into()),
            speaker: None,
            choices: vec![
                Choice::new("What do you think about?", "q2_deep")
                    .sets("affection", 5_i32),
                Choice::new("I like quiet. Tell me about this shipwreck.", "q2_place")
                    .sets("affection", 3_i32),
                Choice::new("Do you ever wish you were a different kind of fish?", "q2_question")
                    .sets("affection", 2_i32),
            ],
        })
        .node(text_node(
            "q2_deep",
            "gill",
            "Everything... and nothing. Like... do fish dream? If we do, what does the ocean dream about? Sometimes I sit in this shipwreck and imagine all the humans who once sailed on it. Where were they going? ...Sorry, is that weird?",
            "q3",
        ))
        .node(text_node(
            "q2_place",
            "gill",
            "I found this place three tides ago. It's quiet. The wood creaks sometimes and it sounds like the ship is breathing. I come here to read the barnacles. ...They tell stories if you look closely.",
            "q3",
        ))
        .node(text_node(
            "q2_question",
            "gill",
            "...Sometimes. But then I think... a swordfish can't puff up. A clownfish can't disappear into the sand. We all have our thing. Mine just happens to be... round.",
            "q3",
        ))
        .node(DialogueNode::Choice {
            id: "q3".into(),
            prompt: Some("The old ship creaks gently around you.".into()),
            speaker: None,
            choices: vec![
                Choice::new("I could listen to you think out loud all night, Gill.", "ending_good")
                    .sets("affection", 6_i32),
                Choice::new("You're a lot deeper than people give you credit for.", "ending_great")
                    .sets("affection", 4_i32),
                Choice::new("This has been... interesting. I should go.", "ending_meh")
                    .sets("affection", 1_i32),
            ],
        })
        .node(text_node(
            "ending_good",
            "gill",
            "*completely deflates to normal size* ...Really? You... you mean that? *tiny smile* ...Next time I'll show you the part of the ship where the starlight comes through the hull. It's... it's beautiful.",
            "end",
        ))
        .node(text_node(
            "ending_great",
            "gill",
            "*blushes* ...Thank you. That means more than you know. Maybe... maybe next time I won't puff up so much. *puffs up* ...Okay maybe a little.",
            "end",
        ))
        .node(text_node(
            "ending_meh",
            "gill",
            "Oh... okay. Yeah. It was nice of you to come. *sinks a little* I'll just... be here. With the ship. It's fine.",
            "end",
        ))
        .node(DialogueNode::end("end"))
        .build_unchecked()
}

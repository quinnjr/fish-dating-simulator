//! Dialogue trees for each dateable fish, built with sable-dialogue.
//!
//! Each fish has 3 dialogue trees that rotate based on how many dates
//! you've been on. Date 1 is sweet and normal. Date 2 gets weird.
//! Date 3 goes full unhinged shitpost energy.

use sable_dialogue::prelude::*;
use sable_dialogue::dialogue::DialogueBuilder;

use crate::data::FishId;
use crate::plugins::FishRegistry;

/// Number of unique dialogues per fish.
const DIALOGUES_PER_FISH: u32 = 3;

/// Build the dialogue tree for a specific fish and date number.
pub fn build_dialogue(fish_id: &FishId, date_number: u32, registry: &FishRegistry) -> DialogueTree {
    match fish_id {
        FishId::Bubbles => {
            let variant = date_number % DIALOGUES_PER_FISH;
            match variant {
                0 => build_bubbles_date1(),
                1 => build_bubbles_date2(),
                _ => build_bubbles_date3(),
            }
        }
        FishId::Marina => {
            let variant = date_number % DIALOGUES_PER_FISH;
            match variant {
                0 => build_marina_date1(),
                1 => build_marina_date2(),
                _ => build_marina_date3(),
            }
        }
        FishId::Gill => {
            let variant = date_number % DIALOGUES_PER_FISH;
            match variant {
                0 => build_gill_date1(),
                1 => build_gill_date2(),
                _ => build_gill_date3(),
            }
        }
        FishId::Plugin(plugin_id) => {
            if let Some(fish) = registry.get(plugin_id) {
                fish.dialogue_for_date(date_number)
            } else {
                // Fallback empty dialogue
                crate::plugins::FishDef::fallback_dialogue_for(fish_id.name())
            }
        }
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

// ═══════════════════════════════════════════════════════════════════════════
//  BUBBLES - Date 1 (Normal sweet clownfish)
// ═══════════════════════════════════════════════════════════════════════════

fn build_bubbles_date1() -> DialogueTree {
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

// ═══════════════════════════════════════════════════════════════════════════
//  BUBBLES - Date 2 (Getting comfortable, chaotic energy emerging)
// ═══════════════════════════════════════════════════════════════════════════

fn build_bubbles_date2() -> DialogueTree {
    DialogueBuilder::new("start")
        .title("Date with Bubbles II")
        .speaker(Speaker::new("bubbles", "Bubbles"))
        .speaker(Speaker::new("player", "You"))
        .node(text_node(
            "start",
            "bubbles",
            "GATHER ROUND my precious little land-dweller because MAMA BUBBLES hath returned with the ENERGY today. I have had three kelp espressos and I am VIBRATING.",
            "q1",
        ))
        .node(DialogueNode::Choice {
            id: "q1".into(),
            prompt: Some("Bubbles is vibrating at a frequency that concerns you.".into()),
            speaker: None,
            choices: vec![
                Choice::new("Three?? Bubbles, your pupils are doing figure eights.", "q1_concern")
                    .sets("affection", 3_i32),
                Choice::new("Match my energy. I had four.", "q1_match")
                    .sets("affection", 5_i32),
                Choice::new("Should I call someone?", "q1_worried"),
            ],
        })
        .node(text_node(
            "q1_concern",
            "bubbles",
            "They're doing INFINITY SYMBOLS actually, which I think means I've achieved enlightenment? Or a medical emergency. Either way I'm having the time of my LIFE.",
            "q2",
        ))
        .node(text_node(
            "q1_match",
            "bubbles",
            "FOUR?! Oh we are DANGEROUS together. This is the energy. THIS IS THE ENERGY RIGHT HERE. Consider this your only warning, the ocean is not ready for us.",
            "q2",
        ))
        .node(text_node(
            "q1_worried",
            "bubbles",
            "Call someone?? Baby I AM the someone people call. I'm entering dangerous thresholds of 'lights up a room' and I should probably fear for my life but I simply choose not to.",
            "q2",
        ))
        .node(DialogueNode::Choice {
            id: "q2".into(),
            prompt: Some("Bubbles pulls out a barnacle-encrusted phone.".into()),
            speaker: None,
            choices: vec![
                Choice::new("What is that.", "q2_phone")
                    .sets("affection", 3_i32),
                Choice::new("Are you posting right now? On a DATE?", "q2_posting")
                    .sets("affection", 4_i32),
                Choice::new("Bubbles. Put the phone down.", "q2_stern")
                    .sets("affection", 1_i32),
            ],
        })
        .node(text_node(
            "q2_phone",
            "bubbles",
            "It's my DEVICE. I found it in a shipwreck and honestly? The ghosts in here post better takes than half the reef. Some fish said pineapple doesn't belong on pizza and I have been SCREAMING for three full minutes.",
            "q3",
        ))
        .node(text_node(
            "q2_posting",
            "bubbles",
            "I'm LIVE-THREADING this date actually. 'Currently on a date with someone who makes my heart do the thing. Thread below.' I've got forty-seven likes. The ocean is INVESTED in us.",
            "q3",
        ))
        .node(text_node(
            "q2_stern",
            "bubbles",
            "I-- okay fine. *puts phone away* *immediately takes it back out* Sorry I lied, this clam influencer just said working from your anemone kills productivity and I need to ratio them IMMEDIATELY.",
            "q3",
        ))
        .node(DialogueNode::Choice {
            id: "q3".into(),
            prompt: Some("Bubbles has somehow gotten louder.".into()),
            speaker: None,
            choices: vec![
                Choice::new("I love that you're completely unhinged.", "ending_good")
                    .sets("affection", 6_i32),
                Choice::new("You're a lot. But you're MY a lot.", "ending_great")
                    .sets("affection", 5_i32),
                Choice::new("I think the kelp espressos were a mistake.", "ending_meh")
                    .sets("affection", 2_i32),
            ],
        })
        .node(text_node(
            "ending_good",
            "bubbles",
            "UNHINGED?? Every time you are nice to me my fins grow even more luscious and powerful. I'm reaching levels of radiance that are genuinely concerning. Thank you for enabling this.",
            "end",
        ))
        .node(text_node(
            "ending_great",
            "bubbles",
            "*slams fin on table* YOURS?? Oh I'm posting about this. 'They called me THEIRS. I am ascending. Goodbye mortal reef.' This is the best date I've EVER had and I've had SEVERAL.",
            "end",
        ))
        .node(text_node(
            "ending_meh",
            "bubbles",
            "A MISTAKE?! The espressos were the only correct decision I made today. The first mistake was when evolution gave me a mouth this powerful. I cannot be stopped, only contained.",
            "end",
        ))
        .node(DialogueNode::end("end"))
        .build_unchecked()
}

// ═══════════════════════════════════════════════════════════════════════════
//  BUBBLES - Date 3 (Full unhinged, maximum shitpost energy)
// ═══════════════════════════════════════════════════════════════════════════

fn build_bubbles_date3() -> DialogueTree {
    DialogueBuilder::new("start")
        .title("Date with Bubbles III")
        .speaker(Speaker::new("bubbles", "Bubbles"))
        .speaker(Speaker::new("player", "You"))
        .node(text_node(
            "start",
            "bubbles",
            "Okay so. I need you to sit down. Actually you're already sitting. I need you to sit down HARDER because I have LORE to drop and the ocean is NOT ready.",
            "q1",
        ))
        .node(DialogueNode::Choice {
            id: "q1".into(),
            prompt: Some("Bubbles has the energy of someone about to reveal a conspiracy.".into()),
            speaker: None,
            choices: vec![
                Choice::new("I'm sitting as hard as I can. Drop the lore.", "q1_ready")
                    .sets("affection", 4_i32),
                Choice::new("Should I be scared?", "q1_scared")
                    .sets("affection", 3_i32),
                Choice::new("Bubbles it's 7 AM.", "q1_tired")
                    .sets("affection", 2_i32),
            ],
        })
        .node(text_node(
            "q1_ready",
            "bubbles",
            "So you know that seahorse that runs the kelp stand on 3rd reef? His name is ALREADY an objectively hilarious name. It's Greg. GREG. A seahorse named GREG. I've been thinking about this for six days and I need someone to validate that this is the funniest thing in the entire ocean.",
            "q2",
        ))
        .node(text_node(
            "q1_scared",
            "bubbles",
            "SCARED?! You should be HONORED. I don't drop lore for just anyone. Last time I told someone a secret this good they physically ascended. Just floated right up. Might have been dead actually. ANYWAY--",
            "q2",
        ))
        .node(text_node(
            "q1_tired",
            "bubbles",
            "AND?! Time is a construct invented by sundials to sell more shadows. I didn't pick up my coral supplements this morning which means I didn't have any to take and I am UNLEASHED. This is me at full power. Consider this your only warning.",
            "q2",
        ))
        .node(DialogueNode::Choice {
            id: "q2".into(),
            prompt: Some("Bubbles is pacing back and forth. A small crowd has gathered.".into()),
            speaker: None,
            choices: vec![
                Choice::new("Bubbles there are fish staring at us.", "q2_crowd")
                    .sets("affection", 4_i32),
                Choice::new("Please continue, I'm taking notes.", "q2_notes")
                    .sets("affection", 5_i32),
                Choice::new("Have you considered therapy?", "q2_therapy")
                    .sets("affection", 2_i32),
            ],
        })
        .node(text_node(
            "q2_crowd",
            "bubbles",
            "Let them STARE. They're here for the SHOW. I've gotta dim this charisma soon. I'm entering dangerous levels of 'brightens every room' and frankly the coral is starting to bleach from proximity to me.",
            "q3",
        ))
        .node(text_node(
            "q2_notes",
            "bubbles",
            "NOTES!! YES!! Document this! Future generations need to know about the time Bubbles the Clownfish single-handedly discovered that the real treasure was the AUDACITY we found along the way. Also Greg the seahorse is a cop, I said what I said.",
            "q3",
        ))
        .node(text_node(
            "q2_therapy",
            "bubbles",
            "Therapy?? Baby I AM therapy. Fish see me swimming by and their serotonin levels SPIKE. I'm not the one who needs help. I'm the help. I'm the whole emergency response team. I'm the 911 of good vibes and I am HERE.",
            "q3",
        ))
        .node(DialogueNode::Choice {
            id: "q3".into(),
            prompt: Some("Bubbles has climbed onto the table. The waiter is concerned.".into()),
            speaker: None,
            choices: vec![
                Choice::new("Never change, Bubbles. The ocean needs you.", "ending_good")
                    .sets("affection", 7_i32),
                Choice::new("You're the main character and everyone knows it.", "ending_great")
                    .sets("affection", 6_i32),
                Choice::new("I'm going to need you to get off the table.", "ending_meh")
                    .sets("affection", 2_i32),
            ],
        })
        .node(text_node(
            "ending_good",
            "bubbles",
            "NEEDS me?! Oh I'm going to CRY. Not sad cry. POWERFUL cry. The kind where a single tear rolls down and a rainbow appears and somewhere a baby dolphin learns to flip. That's the energy you just gave me. I love you. Wait I said that out loud. I'M NOT TAKING IT BACK.",
            "end",
        ))
        .node(text_node(
            "ending_great",
            "bubbles",
            "MAIN CHARACTER?! *knocks over three drinks* PLEASE join me in screaming for three full minutes because THAT is the most correct thing anyone has EVER said. This is canon now. WE are canon. I'm updating my reef bio IMMEDIATELY.",
            "end",
        ))
        .node(text_node(
            "ending_meh",
            "bubbles",
            "The TABLE is my STAGE and this cafe is my VENUE. You can't contain this. Really is a shame that once fish realize the vibes contradict them on suppressing my energy, they lean into 'calm down' like PLEASE stop lying to my face. Just say you can't handle the glow.",
            "end",
        ))
        .node(DialogueNode::end("end"))
        .build_unchecked()
}

// ═══════════════════════════════════════════════════════════════════════════
//  MARINA - Date 1 (Cool, competitive, guarded)
// ═══════════════════════════════════════════════════════════════════════════

fn build_marina_date1() -> DialogueTree {
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

// ═══════════════════════════════════════════════════════════════════════════
//  MARINA - Date 2 (Competitive shitposting, trash talk arc)
// ═══════════════════════════════════════════════════════════════════════════

fn build_marina_date2() -> DialogueTree {
    DialogueBuilder::new("start")
        .title("Date with Marina II")
        .speaker(Speaker::new("marina", "Marina"))
        .speaker(Speaker::new("player", "You"))
        .node(text_node(
            "start",
            "marina",
            "So I raced that barracuda from the north reef today. He talked SO much trash beforehand. Posted about it. 'Marina wouldn't last two seconds in open water.' Oh really? OH REALLY DARREN?",
            "q1",
        ))
        .node(DialogueNode::Choice {
            id: "q1".into(),
            prompt: Some("Marina's eye is twitching slightly. She seems fired up.".into()),
            speaker: None,
            choices: vec![
                Choice::new("What happened?", "q1_ask")
                    .sets("affection", 3_i32),
                Choice::new("His name is Darren? That's already funny.", "q1_name")
                    .sets("affection", 5_i32),
                Choice::new("Are you okay?", "q1_concern")
                    .sets("affection", 2_i32),
            ],
        })
        .node(text_node(
            "q1_ask",
            "marina",
            "I beat him by FOURTEEN body lengths. FOURTEEN. He's currently posting about how 'the current was unfair' and honestly I need everyone to block this guy like his bowels because he clearly refuses to eat fiber.",
            "q2",
        ))
        .node(text_node(
            "q1_name",
            "marina",
            "RIGHT?! We don't even need a trash nickname for him. Darren. DARREN. It's already an objectively hilarious name. A barracuda named DARREN. The jokes write themselves. Do less, nature.",
            "q2",
        ))
        .node(text_node(
            "q1_concern",
            "marina",
            "Am I OKAY? I just achieved the single greatest victory in eastern reef racing history and you're asking if I'm OKAY? I'm not okay. I'm LEGENDARY. There's a difference.",
            "q2",
        ))
        .node(DialogueNode::Choice {
            id: "q2".into(),
            prompt: Some("Marina is now doing victory laps around your table.".into()),
            speaker: None,
            choices: vec![
                Choice::new("What did Darren say after?", "q2_aftermath")
                    .sets("affection", 4_i32),
                Choice::new("I bet I could beat you.", "q2_challenge")
                    .sets("affection", 5_i32),
                Choice::new("This is a date, not a sports debrief.", "q2_date")
                    .sets("affection", 1_i32),
            ],
        })
        .node(text_node(
            "q2_aftermath",
            "marina",
            "He posted a FIVE paragraph essay about 'the state of competitive swimming' and it's just... Darren. My guy. You lost to a fish with a sword on her face. The content writes itself. I screenshot everything.",
            "q3",
        ))
        .node(text_node(
            "q2_challenge",
            "marina",
            "*stops swimming* ...Did you just... You know what? I RESPECT that. Completely delusional confidence. That's MY love language. You'd lose catastrophically but the AUDACITY? Chef's kiss. With a fin.",
            "q3",
        ))
        .node(text_node(
            "q2_date",
            "marina",
            "A sports debrief IS a date when you're dating ME. Really is a shame that once fish realize I'm not going to stop talking about winning, they just lean into the 'can we discuss something else' thing. Like PLEASE. Just say you can't handle glory.",
            "q3",
        ))
        .node(DialogueNode::Choice {
            id: "q3".into(),
            prompt: Some("Marina has pulled up Darren's post on a waterproof tablet.".into()),
            speaker: None,
            choices: vec![
                Choice::new("Ratio him. I'll boost you.", "ending_good")
                    .sets("affection", 6_i32),
                Choice::new("You're terrifying and I'm into it.", "ending_great")
                    .sets("affection", 5_i32),
                Choice::new("I feel bad for Darren honestly.", "ending_meh")
                    .sets("affection", 1_i32),
            ],
        })
        .node(text_node(
            "ending_good",
            "marina",
            "BOOST ME?! *slams sword on table* We are a POWER COUPLE now. Darren doesn't stand a chance against our combined energy. This is the greatest alliance since... I don't know, I don't do history, I do WINNING.",
            "end",
        ))
        .node(text_node(
            "ending_great",
            "marina",
            "Terrifying and you're INTO it. That's it. That's the whole personality test. You passed. Most fish fail right around 'the sword is kind of scary' but you just LEANED IN. I'm keeping you.",
            "end",
        ))
        .node(text_node(
            "ending_meh",
            "marina",
            "Feel BAD for-- Darren posted 'Marina only won because of hydrodynamic privilege' and you feel BAD for him?! We're going to need to have a conversation about where your loyalties lie.",
            "end",
        ))
        .node(DialogueNode::end("end"))
        .build_unchecked()
}

// ═══════════════════════════════════════════════════════════════════════════
//  MARINA - Date 3 (Full villain arc energy, maximum chaos)
// ═══════════════════════════════════════════════════════════════════════════

fn build_marina_date3() -> DialogueTree {
    DialogueBuilder::new("start")
        .title("Date with Marina III")
        .speaker(Speaker::new("marina", "Marina"))
        .speaker(Speaker::new("player", "You"))
        .node(text_node(
            "start",
            "marina",
            "I have been BANNED from the reef racing league. Permanently. They said I was 'too intimidating.' I showed up to qualifiers and three fish forfeited on SIGHT. Apparently that's 'bad for the sport.'",
            "q1",
        ))
        .node(DialogueNode::Choice {
            id: "q1".into(),
            prompt: Some("Marina is vibrating with a rage that feels almost philosophical.".into()),
            speaker: None,
            choices: vec![
                Choice::new("They banned you for being TOO GOOD?", "q1_outrage")
                    .sets("affection", 5_i32),
                Choice::new("To be fair, you did make that one fish cry.", "q1_fair")
                    .sets("affection", 3_i32),
                Choice::new("Maybe this is a sign to find new hobbies.", "q1_hobbies")
                    .sets("affection", 1_i32),
            ],
        })
        .node(text_node(
            "q1_outrage",
            "marina",
            "THANK YOU. This is a clip from 'The Fabulous Life of Eastern Reef Champions' featuring ME and I need everyone to understand that banning excellence is a CHOICE that reflects on THEM not on this sword right here on my FACE.",
            "q2",
        ))
        .node(text_node(
            "q1_fair",
            "marina",
            "He cried because I looked at him. I LOOKED. AT HIM. With my EYES. My regular eyes that are on my regular face. If maintaining eye contact is intimidation then I guess I'm a WAR CRIMINAL, Darren. I GUESS I'M A WAR CRIMINAL.",
            "q2",
        ))
        .node(text_node(
            "q1_hobbies",
            "marina",
            "HOBBIES?! My hobby is being the fastest thing in this ocean and the league just said 'no thank you we prefer mediocrity' like it's a PREFERENCE. You don't get to prefer mediocrity when GREATNESS is right here OFFERING ITSELF.",
            "q2",
        ))
        .node(DialogueNode::Choice {
            id: "q2".into(),
            prompt: Some("Marina has drafted and deleted eleven angry posts.".into()),
            speaker: None,
            choices: vec![
                Choice::new("Start your own league. With your own rules.", "q2_league")
                    .sets("affection", 6_i32),
                Choice::new("What if the real race was the enemies you made along the way?", "q2_profound")
                    .sets("affection", 5_i32),
                Choice::new("Marina this is our third date and I still don't know your favorite color.", "q2_normal")
                    .sets("affection", 2_i32),
            ],
        })
        .node(text_node(
            "q2_league",
            "marina",
            "MY OWN LEAGUE. With MY rules. Rule one: I always win. Rule two: see rule one. Rule three: Darren is banned from even WATCHING. This is the greatest idea anyone has ever had and I'm furious I didn't think of it. Wait. I'm claiming I thought of it.",
            "q3",
        ))
        .node(text_node(
            "q2_profound",
            "marina",
            "*stops pacing* ...That's... actually kind of deep. The enemies I made along the way. *stares into distance* Darren. The league officials. That jellyfish who said I 'try too hard.' You know what? They're all just CHARACTERS in MY story. I'm the PROTAGONIST.",
            "q3",
        ))
        .node(text_node(
            "q2_normal",
            "marina",
            "My favorite-- IT'S WINNING. The color of winning. Which is whatever color I am when I WIN. This is who you signed up to date and I will NOT be diminished by small talk. Ask me something that MATTERS.",
            "q3",
        ))
        .node(DialogueNode::Choice {
            id: "q3".into(),
            prompt: Some("Marina has somehow gotten a whiteboard and is drawing race strategies.".into()),
            speaker: None,
            choices: vec![
                Choice::new("I'd follow you into any battle, Marina. Even against Darren.", "ending_good")
                    .sets("affection", 7_i32),
                Choice::new("You are genuinely the most intense fish alive and I can't look away.", "ending_great")
                    .sets("affection", 6_i32),
                Choice::new("I think you might need to talk to a professional about Darren.", "ending_meh")
                    .sets("affection", 2_i32),
            ],
        })
        .node(text_node(
            "ending_good",
            "marina",
            "INTO BATTLE?! *sword gleaming* We ride at DAWN. Or whenever the tide is right. We ride at... high tide probably. The point is we're riding SOMEWHERE and Darren is going to SEE us and WEEP. You complete me. In a tactical sense.",
            "end",
        ))
        .node(text_node(
            "ending_great",
            "marina",
            "Can't look away? That's called RESPECT and also maybe FEAR and honestly both are valid expressions of love in this economy. I accept your devotion. Now help me workshop this post about Darren, I need it to hit different.",
            "end",
        ))
        .node(text_node(
            "ending_meh",
            "marina",
            "A professional what? A professional WINNER? Because that's what I AM. The Darren situation isn't a PROBLEM it's a NARRATIVE and every good narrative needs an antagonist. He should be THANKING me for making him relevant.",
            "end",
        ))
        .node(DialogueNode::end("end"))
        .build_unchecked()
}

// ═══════════════════════════════════════════════════════════════════════════
//  GILL - Date 1 (Shy, anxious, secretly deep)
// ═══════════════════════════════════════════════════════════════════════════

fn build_gill_date1() -> DialogueTree {
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

// ═══════════════════════════════════════════════════════════════════════════
//  GILL - Date 2 (Coming out of shell, anxious shitposting)
// ═══════════════════════════════════════════════════════════════════════════

fn build_gill_date2() -> DialogueTree {
    DialogueBuilder::new("start")
        .title("Date with Gill II")
        .speaker(Speaker::new("gill", "Gill"))
        .speaker(Speaker::new("player", "You"))
        .node(text_node(
            "start",
            "gill",
            "So um. I... I started posting. On the reef network. Like... my thoughts. Publicly. *puffs up* I have forty-seven followers and I am TERRIFIED.",
            "q1",
        ))
        .node(DialogueNode::Choice {
            id: "q1".into(),
            prompt: Some("Gill is showing you his reef profile. His bio says 'just a fish thinking thoughts. please be gentle.'".into()),
            speaker: None,
            choices: vec![
                Choice::new("Gill that's amazing! What do you post about?", "q1_support")
                    .sets("affection", 4_i32),
                Choice::new("Forty-seven! You're going viral!", "q1_hype")
                    .sets("affection", 5_i32),
                Choice::new("Is that... a lot?", "q1_unsure")
                    .sets("affection", 2_i32),
            ],
        })
        .node(text_node(
            "q1_support",
            "gill",
            "Just... thoughts? Like yesterday I posted 'do crabs know they're walking sideways or do they think everyone else is wrong' and someone REPLIED. A real fish. They said 'I've never thought about this before' and I puffed up so hard I hit the ceiling.",
            "q2",
        ))
        .node(text_node(
            "q1_hype",
            "gill",
            "V-VIRAL?! *MAXIMUM PUFF* Don't say that! One of my posts got twelve likes and I had to lie down in a kelp bed for an hour. If I go viral I will literally, physically, medically explode. That's not hyperbole. I'm a pufferfish. It could happen.",
            "q2",
        ))
        .node(text_node(
            "q1_unsure",
            "gill",
            "I don't know?! Is it?! Forty-seven fish chose to see my thoughts ON PURPOSE. That's forty-seven more than I expected which was ZERO. I'm having a crisis about it. A good crisis? Is that a thing? I think that's a thing.",
            "q2",
        ))
        .node(DialogueNode::Choice {
            id: "q2".into(),
            prompt: Some("Gill is scrolling through his posts nervously.".into()),
            speaker: None,
            choices: vec![
                Choice::new("Read me your favorite post.", "q2_read")
                    .sets("affection", 5_i32),
                Choice::new("Have you gotten any hate?", "q2_hate")
                    .sets("affection", 3_i32),
                Choice::new("You should post more. The ocean needs your energy.", "q2_encourage")
                    .sets("affection", 4_i32),
            ],
        })
        .node(text_node(
            "q2_read",
            "gill",
            "*clears throat* 'I had to pick up my sea supplements today which means I didn't have any to take this morning. So if I seem more puffed than usual please understand I am running on raw, unmedicated Gill right now. This is me at full unfiltered power. I am so sorry.' ...It got twenty-three likes.",
            "q3",
        ))
        .node(text_node(
            "q2_hate",
            "gill",
            "ONE angelfish said my posts are 'too existential for 7 AM' and I immediately puffed up and couldn't fit through my door for three hours. But then?? Twelve fish DEFENDED me?? They said 'let Gill think at whatever hour he wants' and I cried. I'm crying now actually.",
            "q3",
        ))
        .node(text_node(
            "q2_encourage",
            "gill",
            "*stares at you* ...You think the ocean needs... me? My thoughts? The things that rattle around in my weird little pufferfish brain at 3 AM? Like 'what if barnacles have a rich inner life we'll never understand' and 'is sand just really old rocks that gave up'? ...You really think so?",
            "q3",
        ))
        .node(DialogueNode::Choice {
            id: "q3".into(),
            prompt: Some("Gill is typing something. He looks up at you.".into()),
            speaker: None,
            choices: vec![
                Choice::new("Post about tonight. About us. I dare you.", "ending_good")
                    .sets("affection", 7_i32),
                Choice::new("You have a gift, Gill. Own it.", "ending_great")
                    .sets("affection", 5_i32),
                Choice::new("Maybe keep some thoughts private though.", "ending_meh")
                    .sets("affection", 1_i32),
            ],
        })
        .node(text_node(
            "ending_good",
            "gill",
            "*puffs up* *deflates* *puffs up again* ...Okay. *typing* 'Currently on a date with someone who makes me forget to be scared. If you don't hear from me it's because I've ascended. Or my puffing finally reached critical mass. Either way. I'm happy.' ...Posted. *hides face in fins*",
            "end",
        ))
        .node(text_node(
            "ending_great",
            "gill",
            "A... gift? *tiny puff* I've never thought of it as a gift. I thought it was just... the broken parts of my brain leaking out. But if the leaks are... beautiful? Then maybe I'm not broken. Maybe I'm just... dripping with content. Wait that sounds gross. I'm keeping it.",
            "end",
        ))
        .node(text_node(
            "ending_meh",
            "gill",
            "Private? ...Yeah. You're probably right. Some things should stay in the dark parts of the shipwreck. Where nobody can judge them. Where nobody can judge... me. *deflates completely* It's fine. This is fine.",
            "end",
        ))
        .node(DialogueNode::end("end"))
        .build_unchecked()
}

// ═══════════════════════════════════════════════════════════════════════════
//  GILL - Date 3 (Fully unleashed philosopher pufferfish, unhinged in his own quiet way)
// ═══════════════════════════════════════════════════════════════════════════

fn build_gill_date3() -> DialogueTree {
    DialogueBuilder::new("start")
        .title("Date with Gill III")
        .speaker(Speaker::new("gill", "Gill"))
        .speaker(Speaker::new("player", "You"))
        .node(text_node(
            "start",
            "gill",
            "I have two thousand followers now. Two. Thousand. I posted 'what if water is just the sky for ground' at 4 AM and I woke up to CHAOS. Fish are DEBATING it. Marine biologists are WEIGHING IN. I've started something I cannot stop.",
            "q1",
        ))
        .node(DialogueNode::Choice {
            id: "q1".into(),
            prompt: Some("Gill is not puffed up. For the first time, he seems... calm? No. Eerily calm.".into()),
            speaker: None,
            choices: vec![
                Choice::new("You're not puffed up. Are you okay?", "q1_calm")
                    .sets("affection", 4_i32),
                Choice::new("TWO THOUSAND?! Gill you're famous!", "q1_famous")
                    .sets("affection", 5_i32),
                Choice::new("'What if water is just the sky for ground' is objectively incredible.", "q1_validate")
                    .sets("affection", 6_i32),
            ],
        })
        .node(text_node(
            "q1_calm",
            "gill",
            "I've... transcended the puffing. I've been puffed so many times in the last week that I think I used up all my puffs. I'm in the eye of the storm. The calm center of the anxiety hurricane. I have achieved inner peace through sheer exhaustion. It's beautiful and also I might need medical attention.",
            "q2",
        ))
        .node(text_node(
            "q1_famous",
            "gill",
            "Famous. Me. The fish who couldn't order food without puffing up at the waiter. Someone made FAN ART of me. A little drawing of me mid-puff with the caption 'he contains multitudes.' I looked at it and cried for forty-five minutes and then posted about crying and THAT got more likes than the original post.",
            "q2",
        ))
        .node(text_node(
            "q1_validate",
            "gill",
            "Right?? I typed it half asleep and I thought 'this is the dumbest thing I've ever thought' and then I thought 'no actually what IF though' and then I posted it and now a philosophy professor from the deep trench is writing a PAPER about it. I've accidentally become an intellectual. This was never the plan.",
            "q2",
        ))
        .node(DialogueNode::Choice {
            id: "q2".into(),
            prompt: Some("Gill pulls out a list. It's titled 'THINGS I HAVE THOUGHT AT 3 AM (RANKED)'.".into()),
            speaker: None,
            choices: vec![
                Choice::new("Read me the list. All of it.", "q2_list")
                    .sets("affection", 6_i32),
                Choice::new("Gill, are you the ocean's philosopher now?", "q2_philosopher")
                    .sets("affection", 5_i32),
                Choice::new("Do you ever worry you'll run out of thoughts?", "q2_worry")
                    .sets("affection", 3_i32),
            ],
        })
        .node(text_node(
            "q2_list",
            "gill",
            "Number 5: 'if an octopus loses an arm and it grows back, is it the same arm? Does the arm remember?' Number 4: 'sand is just rocks that got too tired to be mountains.' Number 3: 'somewhere right now a fish is living its best day and it doesn't even know.' Number 2: 'we are all just water that learned to be lonely.' And number 1... *deep breath* ...'I think the ocean loves us back.'",
            "q3",
        ))
        .node(text_node(
            "q2_philosopher",
            "gill",
            "I don't WANT to be. I just think things and the things WON'T STOP. It's like my brain is a content machine and the off switch is broken. Yesterday I posted 'is it still swimming if you're not going anywhere' and Bubbles replied 'please join me in screaming for three full minutes' and honestly that energy matched.",
            "q3",
        ))
        .node(text_node(
            "q2_worry",
            "gill",
            "Run out?! The thoughts are INFINITE. That's the problem. Every time I have one thought it splits into four more thoughts. It's thought mitosis. I am MULTIPLYING mentally. There are so many thoughts in here that some of them are just standing in the back going 'we're probably not going to get posted' and that makes me SAD for those thoughts.",
            "q3",
        ))
        .node(DialogueNode::Choice {
            id: "q3".into(),
            prompt: Some("The shipwreck creaks. Gill looks at you with unprecedented clarity.".into()),
            speaker: None,
            choices: vec![
                Choice::new("I think I'm in love with your brain, Gill.", "ending_good")
                    .sets("affection", 8_i32),
                Choice::new("Post about us. I want to be in the lore.", "ending_great")
                    .sets("affection", 6_i32),
                Choice::new("Maybe log off for a bit?", "ending_meh")
                    .sets("affection", 1_i32),
            ],
        })
        .node(text_node(
            "ending_good",
            "gill",
            "*for the first time in three dates, puffs up on purpose* You love my... my brain. The part of me I was most scared of. The loud messy 3 AM thought tornado. You love THAT. *single tear* I'm going to post 'someone loves the broken parts and suddenly they're not broken anymore' and it's going to get so many likes and I deserve every single one.",
            "end",
        ))
        .node(text_node(
            "ending_great",
            "gill",
            "The LORE?! You want to be in MY lore?! *typing furiously* 'Plot twist: the main character found a co-author. The sequel is going to hit different.' Posted. We're canon. The comments are already going insane. Someone said 'GILL HAS A PARTNER?!' and four fish puffed up in solidarity.",
            "end",
        ))
        .node(text_node(
            "ending_meh",
            "gill",
            "Log... off? *thousand yard stare* I can't log off. The thoughts don't stop when I log off. They just... happen without an audience. And if a pufferfish thinks in a shipwreck and nobody's around to read it... did the thought even matter? ...I'm posting that. That's going to SLAP at 2 AM.",
            "end",
        ))
        .node(DialogueNode::end("end"))
        .build_unchecked()
}

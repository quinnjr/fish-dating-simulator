// ═══════════════════════════════════════════════════════════════════════════
//  Coral the Seahorse - Fish Plugin
// ═══════════════════════════════════════════════════════════════════════════
//
//  A flamboyant seahorse fashion influencer who rates everything on a broken
//  numerical scale, bioluminesces uncontrollably when emotional, and has an
//  ongoing blood feud with a sea cucumber named Gerald.
//
//  Plugin API:
//    new_dialogue(title)        -> creates a DialogueDef
//    dialogue.speaker(id, name) -> adds a speaker
//    dialogue.text(id, speaker, text, next) -> adds a text node
//    dialogue.choice(id, prompt, options)   -> adds a choice node
//    dialogue.end(id)           -> adds an end node
//    register_fish(map)         -> registers the fish character

// ═══════════════════════════════════════════════════════════════════════════
//  DATE 1: First Impressions (Sweet, fabulous, the spiral tail era)
// ═══════════════════════════════════════════════════════════════════════════

let date1 = new_dialogue("Date with Coral");
date1.speaker("coral", "Coral");
date1.speaker("player", "You");

date1.text("start", "coral",
    "Oh my absolute STARS you actually came. I need you to understand that I have been getting ready since YESTERDAY. Not because I care what you think. I care what the OCEAN thinks when it sees us together. We are a VISUAL EVENT now. Act accordingly.",
    "q1");

date1.choice("q1", "Coral is posed at a 37-degree angle. You get the feeling this specific angle was rehearsed.", [
    #{ text: "You look absolutely incredible.", next: "q1_good", affection: 4 },
    #{ text: "Did you... rehearse that pose?", next: "q1_funny", affection: 3 },
    #{ text: "Nice restaurant.", next: "q1_meh", affection: 1 },
]);

date1.text("q1_good", "coral",
    "INCREDIBLE. *immediately starts glowing* Oh no. Oh NO. That's the bioluminescence. It happens when I get emotional. I am now a LIGHTHOUSE and it's YOUR fault. Every fish in a 30-foot radius can see me literally SHINING with feelings. I have lost the ability to be subtle. Not that I ever HAD it but the OPTION was nice.",
    "q2");

date1.text("q1_funny", "coral",
    "REHEARSED?! This is NATURAL TALENT. I woke up at 4 AM and stood in front of my anemone mirror for six hours achieving what I like to call 'Deceptive Natural.' That's a term I coined. It means you look effortless but the effort was ASTRONOMICAL. Patent pending. Do NOT steal this. I will find out.",
    "q2");

date1.text("q1_meh", "coral",
    "*blinks* The restaurant. You complimented the BUILDING. While I am RIGHT HERE. Glowing. LITERALLY glowing because I got excited to see you and my stupid bioluminescence activated and now I look like a disco ball with abandonment issues. And you're looking at the WALLS. The WALLS, which I rate a 4. I rate YOU a 6 for this response but you can earn points back. We'll discuss.",
    "q2");

date1.choice("q2", "Coral is reading the menu upside-down and sideways simultaneously. A nearby crab is staring.", [
    #{ text: "Can you actually read that upside-down?", next: "q2_read", affection: 3 },
    #{ text: "Tell me about yourself, Coral.", next: "q2_deep", affection: 5 },
    #{ text: "What do you recommend here?", next: "q2_food", affection: 2 },
]);

date1.text("q2_read", "coral",
    "I can read in ALL orientations. Up, down, sideways, diagonally, at a 47-degree angle, upside-down while spinning. My literacy is OMNIDIRECTIONAL. It's one of my many gifts. The truth is this menu is in a font I don't recognize and I've been squinting at it for five minutes but I'd rather PERISH than admit confusion in public. I'm going to point at something random and whatever it is, that's my DESTINY MEAL. This is how I make all major life decisions.",
    "q3");

date1.text("q2_deep", "coral",
    "*stops posing* Oh. OH. You want to know ME? Not just marvel at me? That's... nobody does that. They see the glow, the tail, the FLAWLESS spiral -- do you see this spiral? This took GENERATIONS of seahorse genetics to perfect and I am the CULMINATION -- but you want the INSIDE? Okay. I'm a seahorse. I carry things. Literally. Did you know seahorse dads carry the babies? I carry babies AND emotional baggage AND the weight of being this beautiful in an ocean that doesn't appreciate fashion. I'm basically a single parent to my own aesthetic.",
    "q3");

date1.text("q2_food", "coral",
    "What do I recommend? EVERYTHING. Because I'M here. My presence elevates all food within a 10-foot radius. That's science. That's not debatable. But specifically? The plankton tartare is *chef's kiss*. Well, *chef's fin tap* because I don't have lips. Which is honestly the ONE design flaw in my entire body and I have THOUGHTS about it. I wrote a letter to evolution. No response. Typical.",
    "q3");

date1.choice("q3", "Coral is now glowing faintly. The waiter is using them as a reading light.", [
    #{ text: "You are genuinely the most fascinating fish in this ocean.", next: "ending_good", affection: 5 },
    #{ text: "I've never laughed this hard on a date.", next: "ending_great", affection: 4 },
    #{ text: "I think I need to get going...", next: "ending_meh", affection: 1 },
]);

date1.text("ending_good", "coral",
    "FASCINATING?! *glow intensifies to stadium floodlight levels* Oh no I'm doing it again. I'm a BEACON. There are fish across the reef squinting going 'what is that light' and the answer is ME having FEELINGS about a COMPLIMENT. I am a one-seahorse light pollution crisis and I CANNOT stop. The waiter just asked me to dim down because I'm 'disturbing the ambiance' and I said excuse me I AM the ambiance, I am generating MORE ambiance than this entire restaurant has EVER produced. ...Same time next week? I'll try to bring a dimmer switch. I won't succeed but I'll TRY.",
    "end");

date1.text("ending_great", "coral",
    "LAUGHED?! Oh that's... that's the best thing you could have said. Better than 'you're beautiful' which is ALSO true but laughter means I got into your BRAIN. I am now occupying real estate in your thought process. That's more intimate than any compliment. I have LODGED myself in your memory. You will think about me at 2 AM and wonder how a seahorse this chaotic exists. I do too. I think about it every day. The answer is the ocean just DOES things sometimes and nobody can stop it.",
    "end");

date1.text("ending_meh", "coral",
    "GET GOING?! In the middle of MY golden hour?! Do you see this light hitting my scales right now? This EXACT lighting happens once every three tides and you want to LEAVE?! I have been WAITING for this bioluminescent-to-ambient-kelp-light ratio since TUESDAY. Fine. FINE. Leave. But know that you are walking away from a 10 out of 10 visual moment and I will NOT be offering makeup sessions. This was a LIMITED ENGAGEMENT and the show is CLOSING and the reviews would have been STELLAR.",
    "end");

date1.end("end");

// ═══════════════════════════════════════════════════════════════════════════
//  DATE 2: The Rating System Arc (Chaos emerging, Gerald enters)
// ═══════════════════════════════════════════════════════════════════════════

let date2 = new_dialogue("Date with Coral II");
date2.speaker("coral", "Coral");
date2.speaker("player", "You");

date2.text("start", "coral",
    "OKAY so before we start I need to address something. I have been rating our first date retrospectively and after careful analysis -- I made a spreadsheet, on a SHELL, with a SHARP ROCK -- you scored an 8.7. Which is historically UNPRECEDENTED. The last fish I dated got a 4.2 and they cried and I felt nothing. You should feel honored. Also I need to tell you about Gerald.",
    "q1");

date2.choice("q1", "Coral has pulled out an actual shell with scratch marks that vaguely resemble a spreadsheet.", [
    #{ text: "Who is Gerald?", next: "q1_gerald", affection: 4 },
    #{ text: "8.7?! What do I need for a 9?", next: "q1_score", affection: 5 },
    #{ text: "You made a shell spreadsheet?", next: "q1_sheet", affection: 3 },
]);

date2.text("q1_gerald", "coral",
    "GERALD. Gerald is a sea cucumber. He lives on the south reef. Last TUESDAY this absolute WALNUT showed up to the kelp garden wearing -- and I need you to sit down for this, actually you're already sitting, sit down HARDER -- wearing the EXACT same seaweed wrap that I designed. MY design. My INTELLECTUAL PROPERTY. He said he 'came up with it independently' like FASHION just APPEARS in sea cucumbers through OSMOSIS. I know for a FACT his friend Derek follows my Seahorse Couture page. This is ESPIONAGE.",
    "q2");

date2.text("q1_score", "coral",
    "A NINE?! Oh you want the full rubric? Okay. Presentation: 9.1 -- you showed up and that's huge. Conversation: 8.4 -- there was a three-second silence where I had to carry and I noticed. Eye contact: 9.8 -- genuinely elite, I almost lost my composure which NEVER happens. Tail compliments: 2.0 -- this is where you TANKED. I have the most PERFECT spiral in the eastern reef and you mentioned it ZERO times. ZERO. The math doesn't lie. Compliment the tail. Fix the deficit. We'll reassess.",
    "q2");

date2.text("q1_sheet", "coral",
    "Yes I made a shell spreadsheet. I make a shell spreadsheet for EVERYTHING. I have one for restaurants, one for current speeds, one for ranking every fish I've ever made eye contact with in order of 'would I bioluminesce.' You're very high on that list. Don't ask where. I'll never tell. Okay FINE you're number one but if you tell anyone I said that I will deny it and rate you a 3 out of spite.",
    "q2");

date2.choice("q2", "Coral is vibrating with the specific frequency of someone who has been wronged by a sea cucumber.", [
    #{ text: "Tell me more about this Gerald situation.", next: "q2_gerald", affection: 5 },
    #{ text: "Rate me. Right now. Real-time scoring.", next: "q2_rate", affection: 4 },
    #{ text: "Coral, this is supposed to be a date.", next: "q2_date", affection: 1 },
]);

date2.text("q2_gerald", "coral",
    "Oh you want the FULL Gerald briefing? STRAP IN. So Gerald -- who by the way is a sea cucumber, which means he's basically a living TUBE with delusions of grandeur -- he posted on the reef network that his kelp wrap was 'avant-garde.' AVANT-GARDE. Sir you are a CYLINDER. You have NO angles. Fashion requires STRUCTURE and you are STRUCTURALLY a pool noodle. I posted a fourteen-paragraph response at 3 AM and Bubbles from the coral reef replied 'this is the most unhinged thing I've ever read' and I said THANK YOU because PASSION looks like UNHINGED to those who have never FELT anything.",
    "q3");

date2.text("q2_rate", "coral",
    "REAL-TIME scoring?! Oh we are DOING this. Okay. Right now. You're at a 9.2. That went up from 8.7 because you asked to be rated which shows SELF-AWARENESS and also a willingness to submit to my judgment which is my NUMBER ONE love language. Oh wait you just tilted your head. 9.3. That was charming. Now you're smiling. 9.4. STOP IT. My scale only goes to 10 and I have STRUCTURAL CONCERNS about what happens if someone exceeds it. I've never had to find out. You're THREATENING my INFRASTRUCTURE.",
    "q3");

date2.text("q2_date", "coral",
    "A DATE?! This IS a date! A date where we DISCUSS things! And the thing we're discussing is that Gerald the sea cucumber STOLE MY DESIGN and the ocean's justice system has NO fashion court and I have been LOBBYING the reef council for months. They said 'Coral, intellectual property doesn't apply to kelp accessories' and I said it DOES when I'm the INTELLECTUAL and the kelp is my PROPERTY. They asked me to leave. I did. DRAMATICALLY. With a full 180-degree tail flourish. The exit was a 10. Even Gerald would rate it a 10 and he rates NOTHING above a 6 because he has the aesthetic sensibility of a ROCK.",
    "q3");

date2.choice("q3", "Coral's bioluminescence is flickering between rage-red and passion-pink. Nearby fish are concerned.", [
    #{ text: "I'll help you destroy Gerald.", next: "ending_good", affection: 6 },
    #{ text: "You are the most intense fish alive and I cannot look away.", next: "ending_great", affection: 5 },
    #{ text: "Maybe Gerald just likes fashion too?", next: "ending_meh", affection: 1 },
]);

date2.text("ending_good", "coral",
    "DESTROY?! *goes full solar flare bioluminescence* You want to help me DESTROY him?! This is the most romantic thing anyone has EVER said to me. More romantic than 'I love you.' More romantic than 'your tail is perfect.' You said 'I will go to WAR with you against a sea cucumber' and my heart just did something it's never done before. I think it BIOLUMINESCED. I didn't know it COULD do that. We are a POWER COUPLE now. Gerald doesn't STAND a chance. Well he doesn't stand at all because he's a tube but the POINT remains. I'm updating your score to 11. Yes the scale only goes to 10. You BROKE IT. I have to recalibrate EVERYTHING now. EVERYTHING.",
    "end");

date2.text("ending_great", "coral",
    "Cannot look AWAY? That's because I am a SPECTACLE. I am a natural phenomenon. Scientists should study why I'm like this and the answer would be 'sometimes the ocean just goes absolutely feral with a design and makes something so aggressively extra that it defies classification.' I am that thing. You're witnessing it in real time. This is like seeing a comet except the comet has a PERFECT spiral tail and opinions about kelp couture. You're welcome. Rating for tonight: 9.8. Highest ever. Don't let it go to your head. Let it go to your HEART.",
    "end");

date2.text("ending_meh", "coral",
    "Maybe Gerald LIKES fashion?! MAYBE HE-- *entire body flashes red* Did you just DEFEND Gerald? On OUR date?! Gerald who STOLE from me? Gerald who wouldn't know haute couture if it WRAPPED around him which IT DID because he STOLE IT FROM ME?! Your score just dropped to a 5. No. A 4. You know what? I'm not even going to finish scoring you. You're UNSCORED. You exist in the void beyond my rating system. That's WORSE than a zero. A zero at least has DEFINITION. You are UNDEFINED. ...I still had a nice time though. Your eyes are pretty. 6.5. I'm conflicted. GOODBYE.",
    "end");

date2.end("end");

// ═══════════════════════════════════════════════════════════════════════════
//  DATE 3: Full Fashion Empire Delusion (Maximum unhinged, the beret saga)
// ═══════════════════════════════════════════════════════════════════════════

let date3 = new_dialogue("Date with Coral III");
date3.speaker("coral", "Coral");
date3.speaker("player", "You");

date3.text("start", "coral",
    "I need you to prepare yourself because I have SEVERAL announcements and each one is more EARTH-SHATTERING than the last. First: Seahorse Couture now has FORTY-THREE followers. Second: I got BANNED from the reef network for 24 hours for posting a 47-paragraph takedown of Gerald's new barnacle bracelet. Third: I am wearing a BERET. It is made of seaweed. It has fallen off fourteen times today. I am NEVER taking it off.",
    "q1");

date3.choice("q1", "Coral is wearing a visibly lopsided seaweed beret. It is sliding. They are pretending it isn't.", [
    #{ text: "The beret is incredible and I need you to know that.", next: "q1_beret", affection: 5 },
    #{ text: "FORTY-THREE? You're an influencer now.", next: "q1_viral", affection: 4 },
    #{ text: "You got banned for a Gerald post?", next: "q1_banned", affection: 6 },
]);

date3.text("q1_beret", "coral",
    "INCREDIBLE. They said INCREDIBLE. *beret immediately falls off* IGNORE THAT. The beret is FINE. It's doing a thing. It's called GRAVITY-DEFIANT ASYMMETRY and it's the next wave in underwater fashion. I didn't DROP it, it REPOSITIONED itself. The beret has AGENCY. Anyway I spent eleven hours making this and the seaweed disintegrated four times and I cried twice and threw it at a wall once and then APOLOGIZED to it because art deserves RESPECT even when it's being DIFFICULT.",
    "q2");

date3.text("q1_viral", "coral",
    "FORTY-THREE followers and counting. I had to turn off notifications because every new follow makes me bioluminesce and my neighbor complained that my apartment was 'a perpetual light show' at 3 AM. You know what I said? I said 'Barbara, creativity doesn't have OFFICE HOURS.' Barbara is an oyster. She's been closed for forty years both physically and EMOTIONALLY. She rated my beret a 2 and I rated her personality a 1. We are at war. I now have TWO enemies. Gerald AND Barbara. The list is growing and I am THRIVING under the pressure.",
    "q2");

date3.text("q1_banned", "coral",
    "BANNED. For 'targeted harassment of a sea cucumber.' TARGETED?! It's not TARGETED if the TARGET deserves it. Let me read you the opening paragraph. *pulls out shell notes* 'Gerald, you absolute TUBE. You brachiating CYLINDER of aesthetic bankruptcy. Your barnacle bracelet looks like someone SNEEZED on a rock and called it jewelry.' That was just the INTRO. I had CHAPTERS. I had CITATIONS. I referenced three fashion publications that don't exist yet because I INVENTED them for the purpose of this takedown. The moderators said it was 'the most thorough and personally devastating critique they'd ever seen' and then banned me anyway. My ban was a 10 out of 10. Even my punishment is EXCELLENT.",
    "q2");

date3.choice("q2", "The beret has fallen off again. Coral kicks it under the table and pretends nothing happened.", [
    #{ text: "Design me something. I want to be part of the empire.", next: "q2_design", affection: 6 },
    #{ text: "What's the endgame for Seahorse Couture?", next: "q2_endgame", affection: 5 },
    #{ text: "Have you considered that Gerald might not be your enemy?", next: "q2_gerald_peace", affection: 1 },
]);

date3.text("q2_design", "coral",
    "FOR YOU?! *MAXIMUM BIOLUMINESCENCE* *beret re-falls off the floor somehow* You want me to design-- Okay. OKAY. I'm seeing it. I'm SEEING the vision. Kelp cape. Shell shoulder pads. Sand dollar brooch right here. A tiny crown made of coral -- not me, the material, although honestly wearing ME as a fashion accessory would also be iconic -- and we WALK into the reef together and every single fish STOPS SWIMMING. Gerald FAINTS. Barbara's shell OPENS for the first time in four decades from the sheer AUDACITY of our coordinated look. We become the ocean's first POWER COUPLE of fashion and the reef network crashes from the engagement. I'm crying. These are DESIGN tears. They're different. They sparkle.",
    "q3");

date3.text("q2_endgame", "coral",
    "ENDGAME?! Seahorse Couture doesn't HAVE an endgame because it has no END. First we dominate the eastern reef. Then the western reef. Then the deep trenches -- which are BEGGING for fashion guidance, they're down there wearing NOTHING but pressure and sadness. Then we go SURFACE. That's right. LAND fashion. Humans think they invented clothes? NO. WE invented the NEED for clothes by being so naturally beautiful that everyone else felt inadequate. I am going to open a boutique. It will be called 'CORAL.' Just one word. In all caps. If you have to explain the brand you've already failed. My forty-three followers will become forty-three THOUSAND and then forty-three MILLION and Gerald will watch from his sad little reef going 'I could have been a part of that' and the answer is NO Gerald. NO you could NOT.",
    "q3");

date3.text("q2_gerald_peace", "coral",
    "Not my ENEMY?! NOT MY-- *bioluminescence goes strobe mode* *beret flies off and hits a passing fish* I need you to understand something. Gerald wore MY design. In PUBLIC. And when I confronted him he said -- and I will remember this until the HEAT DEATH of the ocean -- he said 'Coral, it's just seaweed.' JUST SEAWEED. JUST. SEAWEED. That's like saying the Sistine Chapel is 'just a ceiling.' That's like saying a sunset is 'just the sun leaving.' He reduced my ART to VEGETATION and you want me to make PEACE with that?! My bioluminescence just spelled out the word 'NO' in morse code. That wasn't intentional. My BODY is rejecting your suggestion on a CELLULAR level.",
    "q3");

date3.choice("q3", "Coral has retrieved the beret, put it on backwards, and is pretending this was always the plan.", [
    #{ text: "I want to be Seahorse Couture's co-founder. Design partner. Ride or die.", next: "ending_good", affection: 7 },
    #{ text: "You are the single most unhinged and brilliant fish I have ever known.", next: "ending_great", affection: 6 },
    #{ text: "I think you need to touch kelp. Go outside. Maybe talk to a therapist about Gerald.", next: "ending_meh", affection: 2 },
]);

date3.text("ending_good", "coral",
    "CO-FOUNDER?! *detonates bioluminescently* *beret vaporizes* *the entire restaurant goes bright pink* RIDE OR DIE?! Did you just say RIDE OR DIE about my FASHION EMPIRE?! I am having a MOMENT. This is the moment. The origin story. Decades from now when Seahorse Couture is a global phenomenon and they make the documentary about us, THIS is the scene where the music swells. You said 'ride or die' in a kelp garden while my beret disintegrated from the sheer emotional FORCE of my bioluminescence and I knew -- I KNEW -- that this was the fish I was meant to build an empire with. Gerald is FINISHED. Barbara is IRRELEVANT. We are CORAL AND PARTNER and the ocean will NEVER be the same. I love you. Wait. Did I say that out loud? I said that out loud. I'm rating that confession a 10. I'm rating YOU an 11. I'm rating this MOMENT an infinity. The scale is DESTROYED and I couldn't be HAPPIER about it.",
    "end");

date3.text("ending_great", "coral",
    "Unhinged AND brilliant?! *presses fin to chest* That's the PERFECT combination. That's the DUALITY. The unhinged is the engine and the brilliance is the steering wheel and together they create a vehicle of UNSTOPPABLE creative force that is ME barreling through the ocean at full speed wearing a seaweed beret that won't stay on. You SEE me. Not just the glow, not just the tail -- the WHOLE chaotic, rating-everything, Gerald-hating, fashion-obsessed, bioluminescent DISASTER that I am. And you said BRILLIANT. *starts crying* *starts glowing* *the crying makes the glowing worse* I'm a MESS but I'm a BRILLIANT mess and for the first time someone GETS IT. Your final rating is: beyond measurement. You have transcended the system. You are the first fish to exist outside my spreadsheet and I think that means I'm in love. Don't tell Gerald.",
    "end");

date3.text("ending_meh", "coral",
    "TOUCH KELP?! I am WEARING kelp! It's a BERET! And you want me to talk to a THERAPIST about Gerald?! I don't need a therapist. I need JUSTICE. I need the reef council to establish a fashion tribunal. I need a RESTRAINING ORDER that prevents Gerald from wearing anything I've ever CONCEIVED of, which is everything, because my creative range is INFINITE. You want me to go OUTSIDE?! This IS outside! We're in the OCEAN! There is no MORE outside than this! Your score for tonight is a 3. No. A 2.5. But the evening gets a 7 because the bioluminescence was particularly good tonight and the beret fell off in a way that was somehow still elegant. I contain MULTITUDES even in rejection. Goodnight. I hope you think about what you've said. I'm going to go home and rate this conversation on my shell spreadsheet and the column for 'emotional damage' is going to be OFF THE CHARTS.",
    "end");

date3.end("end");

// ── Register the fish ─────────────────────────────────────────────────────

register_fish(#{
    id: "coral_seahorse",
    name: "Coral",
    species: "Seahorse",
    description: "A bioluminescent fashion influencer seahorse at war with a sea cucumber named Gerald.",
    difficulty: 0.4,
    color: [1.0, 0.4, 0.7, 1.0],

    art_happy: `    .---.
   / ^o^ \
   | <3  |
   \ ~~~ /
    )   (
   /  S  \
  |  / \  |
   \/   \/`,

    art_neutral: `    .---.
   / o.o \
   | --- |
   \ ~~~ /
    )   (
   /  S  \
  |  / \  |
   \/   \/`,

    art_sad: `    .---.
   / ;.; \
   | --- |
   \ ~~~ /
    )   (
   /  S  \
  |  / \  |
   \/   \/`,

    art_small: "><S>",

    pond_name: "Kelp Garden Shallows",
    date_location: "Kelp Fashion Garden",
    date_scene_art: `   ~~~~/\~~~~~/\~~~~
  ~~/  \~~~/  \~~~
 ~/    \~/    \~~
~/ KELP \ KELP \~
~~~~~~~~^^^^^^^^~~~
 ~~~ FASHION ~~~
  ~~~ GARDEN ~~~`,

    dates: [date1, date2, date3],
});

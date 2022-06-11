import { registerTypeDesc } from "../components/ValueTypes";
import { getOptionName, getOptionOf } from "./Option";

export const EMOTION = {
    name: 'Emotion',
    variants: 'Sadness,Happiness,Surprise,Anger,Stress,Fear,Disgust'.split(',').map(name => ({ name })),
};

export const SONG = {
    name: 'Song',
    variants: 'Origin,Charity,Fascination,Youth,Feasting,Decadence,Unheard,Sorrow,Merriment,Doom,Foundation,Destiny,Tranquility,Awakening,Harmony,Remembrance,Hero,Mythics,Fate,Oblivion'.split(',').map(name => ({ name })),
};

export const WEAVABLE = {
    name: 'Weavable',
    variants: 'Anelace'.split(',').map(name => ({ name })),
};

export const WEAVING_ATTACK = {
    name: 'WeavingAttack',
    variants: 'Tearing,Soundblast,Globes,Swindle,Barbs,Polarity,Effigy,Runeband,Bladestorm,Ironcollar,Headstitch,Heartcage'.split(',').map(name => ({ name })),
};

export const PERFORMANCE_ATTACK = {
    name: 'PerformanceAttack',
    variants: [{
        name: 'TempoOne',
        fields: ['String'],
    }, {
        name: 'TempoTwo',
        fields: ['String', 'String'],
    }, {
        name: 'TempoThree',
        fields: ['String', 'String', 'String'],
    }, {
        name: 'Needle',
        fields: ['String'],
    }, {
        name: 'Harry',
        fields: ['String'],
    }, {
        name: 'Bravado',
        fields: ['String'],
    }, {
        name: 'Anelace',
    }, {
        name: 'Pierce',
    }, {
        name: 'Seduce',
    }, {
        name: 'Guilt',
    }, {
        name: 'Ridicule',
    }, {
        name: 'Crackshot',
    }, {
        name: 'Quip',
    }, {
        name: 'Sock',
    }, {
        name: 'Hiltblow',
    }, {
        name: 'Cadence',
    }]
};

export const VENOM_ATTACK = {
    name: 'BardVenomAttack',
    variants: 'Tempo,Needle,Harry,Bravado'.split(',').map(name => ({ name })),
};

registerTypeDesc(EMOTION);
registerTypeDesc(SONG);
registerTypeDesc(getOptionOf(SONG));
registerTypeDesc(WEAVABLE);
registerTypeDesc(WEAVING_ATTACK);
registerTypeDesc(PERFORMANCE_ATTACK);
registerTypeDesc(VENOM_ATTACK);

export const BARD_PREDICATE = {
    name: 'BardPredicate',
    variants: [{
        name: 'Undithered',
    }, {
        name: 'InRhythm',
    }, {
        name: 'InHalfBeat',
    }, {
        name: 'InWholeBeat',
    }, {
        name: 'Runebanded',
    }, {
        name: 'IronCollared',
    }, {
        name: 'Globed',
    }, {
        name: 'GlobeAffIsPriority',
    }, {
        name: 'Awakened',
    }, {
        name: 'Bladestorm',
    }, {
        name: 'HasAnelace',
        fields: [getOptionName('usize')],
    }, {
        name: 'PrimaryEmotion',
        fields: ['Emotion'],
    }, {
        name: 'EmotionLevel',
        fields: ['Emotion', 'usize'],
    }, {
        name: 'Needled',
        fields: [getOptionName('String')],
    }, {
        name: 'NeedlingFor',
        fields: ['FType'],
    }, {
        name: 'Singing',
        fields: [getOptionName('Song')],
    }, {
        name: 'Playing',
        fields: [getOptionName('Song')],
    }]
};

registerTypeDesc(BARD_PREDICATE);

export const BARD_BEHAVIOR = {
    name: 'BardBehavior',
    variants: [{
        name: 'Weave',
        fields: ['Weavable'],
    }, {
        "name": "Anelace",
    }, {
        "name": "ColdRead",
    }, {
        name: 'WeaveAttack',
        fields: ['WeavingAttack'],
    }, {
        name: 'PerformanceAttack',
        fields: ['PerformanceAttack'],
    }, {
        name: 'VenomAttack',
        fields: ['BardVenomAttack'],
    }, {
        name: 'SingSong',
        fields: ['Song'],
    }, {
        name: 'PlaySong',
        fields: ['Song'],
    }],
};

registerTypeDesc(BARD_BEHAVIOR);
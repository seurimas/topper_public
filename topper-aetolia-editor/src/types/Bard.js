import { registerTypeDesc } from "../components/ValueTypes";

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
    variants: ''.split(',').map(name => ({ name })),
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

registerTypeDesc(EMOTION);
registerTypeDesc(SONG);
registerTypeDesc(WEAVABLE);
registerTypeDesc(WEAVING_ATTACK);
registerTypeDesc(PERFORMANCE_ATTACK);

export const BARD_PREDICATE = {
    name: 'BardPredicate',
    variants: [{
        name: 'InHalfBeat',
    }, {
        name: 'InWholeBeat',
    }, {
        name: 'Runebanded',
    }, {
        name: 'Globed',
    }, {
        name: 'Awakened',
    }, {
        name: 'Bladestorm',
    }, {
        name: 'PrimaryEmotion',
        fields: ['Emotion'],
    }, {
        name: 'EmotionLevel',
        fields: ['Emotion', 'usize'],
    }, {
        name: 'Singing',
        fields: ['Song'],
    }, {
        name: 'Playing',
        fields: ['Song'],
    }]
};

registerTypeDesc(BARD_PREDICATE);

export const BARD_BEHAVIOR = {
    name: 'BardBehavior',
    variants: [{
        name: 'Weave',
        fields: ['Weavable'],
    }, {
        name: 'WeaveAttack',
        fields: ['WeavingAttack'],
    }, {
        name: 'PerformanceAttack',
        fields: ['PerformanceAttack'],
    }, {
        name: 'SingSong',
        fields: ['Song'],
    }, {
        name: 'PlaySong',
        fields: ['Song'],
    }],
};

registerTypeDesc(BARD_BEHAVIOR);
import { registerTypeDesc } from "../components/ValueTypes";
import './Bard';
import './Common';
import './FType';

export const UNPOWERED_TREE_DEF_DESC = {
    name: 'UnpoweredTreeDef',
    renderer: 'Enum',
    variants: [
        {
            name: 'Sequence',
            fields: [{
                name: 'Vec',
                itemType: 'UnpoweredTreeDef',
            }],
        },
        {
            name: 'Selector',
            fields: [{
                name: 'Vec',
                itemType: 'UnpoweredTreeDef',
            }],
        },
        {
            name: 'Repeat',
            fields: ['UnpoweredTreeDef', 'usize'],
        },
        {
            name: 'RepeatUntilFail',
            fields: ['UnpoweredTreeDef'],
        },
        {
            name: 'User',
            fields: ['AetBehaviorTreeNode']
        }
    ],
};

registerTypeDesc(UNPOWERED_TREE_DEF_DESC);

export const AET_BEHAVIOR_TREE_NODE = {
    name: 'AetBehaviorTreeNode',
    variants: [{
        name: 'Action',
        fields: ['AetBehavior'],
    }, {
        name: 'Predicate',
        fields: ['AetPredicate'],
    }],
};

registerTypeDesc(AET_BEHAVIOR_TREE_NODE);

export const AET_BEHAVIOR = {
    name: 'AetBehavior',
    variants: [{
        name: 'PlainQebBehavior',
        fields: ['String'],
    }, {
        name: 'BardBehavior',
        fields: ['BardBehavior'],
    }],
};

registerTypeDesc(AET_BEHAVIOR);

export const AET_PREDICATE = {
    name: 'AetPredicate',
    variants: [{
        name: 'AllAffs',
        fields: ['AetTarget', {
            name: 'Vec',
            itemType: 'FType',
        }],
    }, {
        name: 'SomeAffs',
        fields: ['AetTarget', {
            name: 'Vec',
            itemType: 'FType',
        }],
    }, {
        name: 'NoAffs',
        fields: ['AetTarget', {
            name: 'Vec',
            itemType: 'FType',
        }],
    }, {
        name: 'BardPredicate',
        fields: ['AetTarget', 'BardPredicate'],
    }]
}

registerTypeDesc(AET_PREDICATE);

export const AET_TARGET = {
    name: 'AetTarget',
    variants: [{
        name: 'Me',
    }, {
        name: 'Target',
    }],
};

registerTypeDesc(AET_TARGET);
import { registerTypeDesc } from "../components/ValueTypes";

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
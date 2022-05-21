import { TYPE_DESCS } from "../components/ValueTypes";
import { VEC_TYPE_DESC } from "../types/Common";
import { UNPOWERED_TREE_DEF_DESC } from "../types/TreeDef";
import { ADD_TO_VEC, REMOVE_FROM_VEC, SET_ENUM_VARIANT, SET_VALUE } from "./actions";

const getEmptyVec = (itemType) => ({
    name: 'Vec',
    fields: [],
    itemType,
});

const getDefaultValue = (field) => {
    if (typeof field === 'string') {
        return getDefaultValue(TYPE_DESCS[field]);
    } else if (field.name === 'Vec') {
        console.log(field);
        return getEmptyVec(field.itemType);
    } else if (field.defaultValue) {
        return field.defaultValue;
    } else {
        return '';
    }
};

const initState = {
    name: UNPOWERED_TREE_DEF_DESC.name,
    variant: UNPOWERED_TREE_DEF_DESC.variants[0],
    fields: UNPOWERED_TREE_DEF_DESC.variants[0].fields.map(getDefaultValue),
};

export const behaviorTreeReducer = (state = initState, action) => {
    switch (action.type) {
        case ADD_TO_VEC:
            return increaseVecSize(action.path, state);
        case REMOVE_FROM_VEC:
            return deleteVecItem(action.path, state, action.index);
        case SET_VALUE:
            return updateValue(action.path, state, action.value);
        case SET_ENUM_VARIANT:
            return updateEnumVariant(action.path, state, action.variant);
        default:
            return state;
    }
};

const updateTree = (path, behaviorTree, update) => {
    const newState = { ...behaviorTree };
    const subState = path.reduce((tree, idx) => {
        const nextTree = {
            ...tree.fields[idx],
        };
        tree.fields = [...tree.fields];
        tree.fields.splice(idx, 1, nextTree);
        return nextTree;
    }, newState);
    update(subState);
    return newState;
}

const updateEnumVariant = (path, behaviorTree, variant) => {
    return updateTree(path, behaviorTree, (subState) => {
        subState.variant = variant;
        subState.fields = variant.fields.map(getDefaultValue);
        return subState;
    });
};

export const parentPath = (path) => path.slice(0, path.length - 1);
export const leafPath = (path) => path[path.length - 1];

const updateValue = (path, behaviorTree, value) => {
    return updateTree(parentPath(path), behaviorTree, (subState) => {
        subState.fields[leafPath(path)] = value;
    });
};

const increaseVecSize = (path, behaviorTree) => {
    console.log(path, behaviorTree);
    return updateTree(path, behaviorTree, (subState) => {
        console.log(subState);
        subState.fields = [...subState.fields, getDefaultValue(subState.itemType)];
    });
};

const deleteVecItem = (path, behaviorTree, index) => {
    return updateTree(path, behaviorTree, (subState) => {
        subState.fields = subState.fields.filter((_, idx) => index !== idx);
    });
};

const getElement = (path, behaviorTree) => path.reduce((tree, idx) => {
    return tree.fields && tree.fields[idx] || getEmptyVec();
}, behaviorTree);

export const getEnumVariant = (path) => ({ behaviorTree }) => {
    const element = getElement(path, behaviorTree);
    return element
        ? element.variant
        : null;
};

export const getEnumField = (path) => ({ behaviorTree }) => {
    const parent = getElement(parentPath(path), behaviorTree);
    return parent.fields[leafPath(path)] || '';
};

export const getVecPaths = (path) => ({ behaviorTree }) =>
    getElement(path, behaviorTree).fields.map((_, idx) => [...path, idx]);
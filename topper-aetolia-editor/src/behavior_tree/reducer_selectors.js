import { getTypeFromVariant, TYPE_DESCS } from "../components/ValueTypes";
import { VEC_TYPE_DESC } from "../types/Common";
import { UNPOWERED_TREE_DEF_DESC } from "../types/TreeDef";
import { ADD_TO_VEC, CREATE_NEW_TREE, LOAD_JSON, REMOVE_FROM_VEC, SET_ENUM_VARIANT, SET_VALUE } from "./actions";

const getEmptyVec = (itemType) => ({
    name: 'Vec',
    fields: [],
    itemType,
});

const getEmptyTree = () => ({
    name: UNPOWERED_TREE_DEF_DESC.name,
    variant: UNPOWERED_TREE_DEF_DESC.variants[0],
    fields: UNPOWERED_TREE_DEF_DESC.variants[0].fields.map(getDefaultValue),
});

const getDefaultValue = (field) => {
    if (typeof field === 'string') {
        if (field === 'String') {
            return '';
        } else {
            return getDefaultValue(TYPE_DESCS[field]);
        }
    } else if (field.name === 'Vec') {
        return getEmptyVec(field.itemType);
    } else if (field.defaultValue) {
        return field.defaultValue;
    } else {
        return '';
    }
};

const initState = {};

export const behaviorTreeReducer = (state = initState, action) => {
    switch (action.type) {
        case CREATE_NEW_TREE:
            return {
                ...state,
                [action.treeName]: getEmptyTree(),
            }
        case ADD_TO_VEC:
            return increaseVecSize(action.treeName, action.path, state);
        case REMOVE_FROM_VEC:
            return deleteVecItem(action.treeName, action.path, state, action.index);
        case SET_VALUE:
            return updateValue(action.treeName, action.path, state, action.value);
        case SET_ENUM_VARIANT:
            return updateEnumVariant(action.treeName, action.path, state, action.variant);
        case LOAD_JSON:
            return {
                ...state,
                [action.treeName]: hydrateJson(action.json),
            };
        default:
            return state;
    }
};

const updateTree = (treeName, path, behaviorTree, update) => {
    const newState = {
        ...behaviorTree,
        [treeName]: { ...behaviorTree[treeName] }
    };
    const subState = path.reduce((tree, idx) => {
        const nextTree = {
            ...tree.fields[idx],
        };
        tree.fields = [...tree.fields];
        tree.fields.splice(idx, 1, nextTree);
        return nextTree;
    }, newState[treeName]);
    update(subState);
    return newState;
}

const updateEnumVariant = (tree, path, behaviorTree, variant) => {
    return updateTree(tree, path, behaviorTree, (subState) => {
        subState.variant = variant;
        subState.fields = variant.fields && variant.fields.map(getDefaultValue);
        return subState;
    });
};

export const parentPath = (path) => path.slice(0, path.length - 1);
export const leafPath = (path) => path[path.length - 1];

const updateValue = (tree, path, behaviorTree, value) => {
    return updateTree(tree, parentPath(path), behaviorTree, (subState) => {
        subState.fields[leafPath(path)] = value;
    });
};

const increaseVecSize = (tree, path, behaviorTree) => {
    return updateTree(tree, path, behaviorTree, (subState) => {
        subState.fields = [...subState.fields, getDefaultValue(subState.itemType)];
    });
};

const deleteVecItem = (tree, path, behaviorTree, index) => {
    return updateTree(tree, path, behaviorTree, (subState) => {
        subState.fields = subState.fields.filter((_, idx) => index !== idx);
    });
};

export const getTreeNames = ({ behaviorTree }) => Object.keys(behaviorTree);

const getElement = (treeName, path, behaviorTrees) => path.reduce((tree, idx) => {
    return tree.fields && tree.fields[idx];
}, behaviorTrees[treeName]);

export const getEnumVariant = (treeName, path) => ({ behaviorTree }) => {
    const element = getElement(treeName, path, behaviorTree);
    return element
        ? element.variant
        : null;
};

export const getEnumField = (treeName, path) => ({ behaviorTree }) => {
    return getElement(treeName, path, behaviorTree);
};

export const getVecPaths = (treeName, path) => ({ behaviorTree }) =>
    getElement(treeName, path, behaviorTree).fields.map((_, idx) => [...path, idx]);

export const concentrate = (tree, typeDesc) => {
    if (tree.variant) {
        // Enum value.
        if (!tree.fields || tree.fields.length === 0) {
            // Unit.
            return tree.variant.name;
        } else if (tree.fields.length === 1) {
            if (tree.variant.name === "Some") {
                return concentrate(tree.fields[0], tree.variant.fields[0]);
            }
            // Tuple N = 1.
            return {
                [tree.variant.name]: concentrate(tree.fields[0], tree.variant.fields[0]),
            };
        } else {
            // Tuple N = 2+
            return {
                [tree.variant.name]: tree.fields.map((treeField, fieldIdx) => concentrate(treeField, tree.variant.fields[fieldIdx])),
            };
        }
        // TODO Tuple with field names.
    } else if (tree.fields) {
        // Vec
        return tree.fields.map((treeField) => concentrate(treeField, typeDesc.itemType));
    } else if (typeDesc && typeDesc === 'usize') {
        return Number.parseInt(tree);
    } else {
        return tree;
    }
};

export const getJsonOutput = (treeName) => ({ behaviorTree }) => {
    return JSON.stringify(concentrate(behaviorTree[treeName]));
};

export const hydrateVariant = (concentrated, typeDesc) => {
    let variantName = "None";
    if (concentrated !== null) {
        variantName = typeof concentrated === 'object'
            ? Object.keys(concentrated)[0]
            : concentrated;
    }
    const variant = typeDesc.variants.find(({ name }) => name === variantName);
    if (variant.fields) {
        const values = concentrated[variantName];
        console.log(concentrated, variantName);
        if (variant.fields.length > 1) {
            return {
                variant,
                fields: variant.fields.map((fieldDesc, idx) => hydrateField(values[idx], fieldDesc)),
            };
        } else {
            return {
                variant,
                fields: [hydrateField(values, variant.fields[0])],
            };
        }
    } else {
        return {
            variant,
        };
    }
};

export const hydrateVec = (concentrated, itemType) => {
    return {
        name: 'Vec',
        fields: concentrated.map((item) => hydrateField(item, itemType)),
        itemType,
    };
};

export const hydrateField = (concentrated, fieldDesc) => {
    console.log(concentrated, fieldDesc);
    if (typeof fieldDesc === 'string') {
        const typeDesc = TYPE_DESCS[fieldDesc];
        if (typeDesc) {
            return hydrateField(concentrated, typeDesc);
        } else {
            return concentrated;
        }
    } else if (fieldDesc.name && fieldDesc.name === 'Vec') {
        return hydrateVec(concentrated, fieldDesc.itemType);
    } else if (fieldDesc.name === 'usize') {
        return "" + concentrated;
    } else if (fieldDesc.name && fieldDesc.name.indexOf("Option") === 0) {
        if (concentrated === null) {
            return hydrateVariant(concentrated, fieldDesc);
        } else {
            return hydrateVariant({ Some: concentrated }, fieldDesc);
        }
    } else if (fieldDesc.variants) {
        return hydrateVariant(concentrated, fieldDesc);
    } else {
        return concentrated;
    }
};

export const hydrateJson = (json) => {
    const concentratedTree = JSON.parse(json);
    return hydrateVariant(concentratedTree, UNPOWERED_TREE_DEF_DESC);
};
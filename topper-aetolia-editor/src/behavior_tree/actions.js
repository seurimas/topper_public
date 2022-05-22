export const CREATE_NEW_TREE = 'CREATE_NEW_TREE';

export const ADD_TO_VEC = 'ADD_TO_VEC';
export const REMOVE_FROM_VEC = 'REMOVE_FROM_VEC';

export const SET_ENUM_VARIANT = 'SET_ENUM_VARIANT';
export const SET_VALUE = 'SET_VALUE';

export const LOAD_JSON = 'LOAD_JSON';

export const createNewTree = (treeName) => ({
    type: CREATE_NEW_TREE,
    treeName,
});

export const addToVec = (treeName, path) => ({
    type: ADD_TO_VEC,
    treeName,
    path,
});

export const removeFromVec = (treeName, path, index) => ({
    type: REMOVE_FROM_VEC,
    treeName,
    path,
    index,
});

export const setEnumVariant = (treeName, path, variant) => ({
    type: SET_ENUM_VARIANT,
    treeName,
    path,
    variant,
});

export const setValue = (treeName, path, value) => ({
    type: SET_VALUE,
    treeName,
    path,
    value,
});

export const loadJson = (treeName, json) => ({
    type: LOAD_JSON,
    treeName,
    json,
})
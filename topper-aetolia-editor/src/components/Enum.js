import { FormControl, InputLabel, MenuItem, Select } from '@mui/material';
import React, { Fragment } from 'react';
import { useSelector, useDispatch } from 'react-redux';
import { setEnumVariant } from '../behavior_tree/actions';
import { getEnumVariant } from '../behavior_tree/reducer_selectors';
import { registerTypeRenderer, renderValueOfType } from './ValueTypes';

const isUnit = (variantDesc) => variantDesc && (!variantDesc.fields || variantDesc.fields.length === 0);
const isTuple = (variantDesc) => variantDesc && variantDesc.fields && !!variantDesc.fields.length;

export const EnumDropdown = ({
    treeName, path, typeDesc,
}) => {
    const discriminants = typeDesc.variants.map(({ name }) => name);
    const dispatch = useDispatch();
    const value = useSelector(getEnumVariant(treeName, path));
    const setValue = (event) => {
        const newVariantName = event.target.value;
        const newVariant = typeDesc.variants.find(({ name }) => name === newVariantName);
        dispatch(setEnumVariant(treeName, path, newVariant));
    };
    const menuItems = discriminants.map((name) => (
        <MenuItem value={name}>{name}</MenuItem>
    ));
    return (
        <FormControl>
            <InputLabel>{typeDesc.name}</InputLabel>
            <Select value={value ? value.name : null} onChange={setValue}>
                {menuItems}
            </Select>
        </FormControl>
    );
};

export const Enum = ({
    treeName, path, typeDesc,
}) => {
    const discriminantSelector = <EnumDropdown
        treeName={treeName}
        path={path}
        typeDesc={typeDesc}
    />;
    let fields = [];
    const variantDesc = useSelector(getEnumVariant(treeName, path));
    if (isTuple(variantDesc)) {
        fields = variantDesc.fields.map((fieldTypeDesc, idx) => {
            return renderValueOfType(treeName, [...path, idx], fieldTypeDesc);
        });
        return (<>
            {discriminantSelector}
            ({fields})
        </>);

    }
    return discriminantSelector;
};

registerTypeRenderer("Enum", Enum);
import { FormControl, InputLabel, MenuItem, Select } from '@mui/material';
import React, { Fragment } from 'react';
import { useSelector, useDispatch } from 'react-redux';
import { setEnumVariant } from '../behavior_tree/actions';
import { getEnumVariant } from '../behavior_tree/reducer_selectors';
import { registerTypeRenderer, renderValueOfType } from './ValueTypes';

const isUnit = (variantDesc) => variantDesc && (!variantDesc.fields || variantDesc.fields.length === 0);
const isTuple = (variantDesc) => variantDesc && variantDesc.fields && !!variantDesc.fields.length;
const isNamed = (variantDesc) => !isUnit(variantDesc) && !isTuple(variantDesc);

export const EnumDropdown = ({
    path, typeDesc,
}) => {
    const discriminants = typeDesc.variants.map(({ name }) => name);
    const dispatch = useDispatch();
    const value = useSelector(getEnumVariant(path));
    const setValue = (event) => {
        const newVariantName = event.target.value;
        const newVariant = typeDesc.variants.find(({ name }) => name === newVariantName);
        dispatch(setEnumVariant(path, newVariant));
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
    path, typeDesc,
}) => {
    console.log(path, typeDesc);
    const discriminantSelector = <EnumDropdown
        path={path}
        typeDesc={typeDesc}
    />;
    let fields = [];
    const variantDesc = useSelector(getEnumVariant(path));
    if (isTuple(variantDesc)) {
        fields = variantDesc.fields.map((fieldTypeDesc, idx) => {
            return renderValueOfType([...path, idx], fieldTypeDesc);
        });
    }
    return (<>
        {discriminantSelector}
        {fields}
    </>
    );
};

registerTypeRenderer("Enum", Enum);
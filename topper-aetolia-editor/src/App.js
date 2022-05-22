import React, { useState } from 'react';
import logo from './logo.svg';
import './App.scss';
import { EnumDropdown } from './components/Enum';
import { VecList } from './components/Vec';
import { renderValueOfType } from './components/ValueTypes';
import { UNPOWERED_TREE_DEF_DESC } from './types/TreeDef';
import { JsonOutput } from './behavior_tree/JsonOutput';
import { useDispatch, useSelector } from 'react-redux';
import { getTreeNames } from './behavior_tree/reducer_selectors';
import { Accordion, AccordionDetails, AccordionSummary, IconButton, TextField } from '@mui/material';
import { createNewTree } from './behavior_tree/actions';
import { Add } from '@mui/icons-material';

const NewTreeForm = () => {
  const dispatch = useDispatch();
  const [treeName, setTreeName] = useState('');
  const onChange = (event) => setTreeName(event.target.value);
  const spawnTree = () => dispatch(createNewTree(treeName));
  return (
    <>
      <TextField label="New Tree Name" onChange={onChange} />
      <IconButton color="primary" onClick={spawnTree}><Add /></IconButton>
    </>
  );
};

function App() {
  const treeNames = useSelector(getTreeNames);
  console.log(treeNames);
  return (
    <div className="App">
      {treeNames.map((treeName) => <Accordion>
        <AccordionSummary>{treeName}</AccordionSummary>
        <AccordionDetails>
          <JsonOutput treeName={treeName} />
          {renderValueOfType(treeName, [], UNPOWERED_TREE_DEF_DESC)}
        </AccordionDetails>
      </Accordion>
      )}
      <NewTreeForm />
    </div>
  );
}

export default App;

import logo from './logo.svg';
import './App.scss';
import { EnumDropdown } from './components/Enum';
import { VecList } from './components/Vec';
import { renderValueOfType } from './components/ValueTypes';
import { UNPOWERED_TREE_DEF_DESC } from './types/TreeDef';
import { JsonOutput } from './behavior_tree/JsonOutput';

function App() {
  return (
    <div className="App">
      <JsonOutput />
      {renderValueOfType([], UNPOWERED_TREE_DEF_DESC)}
    </div>
  );
}

export default App;

import logo from './logo.svg';
import './App.scss';
import { EnumDropdown } from './components/Enum';
import { VecList } from './components/Vec';
import { renderValueOfType } from './components/ValueTypes';
import { UNPOWERED_TREE_DEF_DESC } from './types/TreeDef';

function App() {
  return (
    <div className="App">
      {renderValueOfType([], UNPOWERED_TREE_DEF_DESC)}
    </div>
  );
}

export default App;

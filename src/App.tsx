import Decryptor from "./components/Decryptor/Decryptor";
import InputEncryptor from "./components/InputEncryptor/InputEncryptor";
import KeySelector from "./components/KeySelector/KeySelector";
import "./style.scss";

function App() {

  return (
    <div id="app">
      <KeySelector />
      <InputEncryptor />
      <Decryptor />
    </div>
  );
}

export default App;

import logo from './logo.svg';
import styles from './App.module.css';
import Input from './Input';
import WordList from './WordList';

function App() {
  return (
    <div class="container">
      <div class="title">Broca</div>

      <Input></Input>

      <WordList></WordList>
    </div>
  );
}

export default App;

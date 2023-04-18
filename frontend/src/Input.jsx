import styles from './Input.module.css';


function Input() {
    return (
        <div class={styles['webflow-style-input']}>
          <input class="" type="text" placeholder="What's your email?"></input>
          <button type="submit"><i class="icon ion-android-arrow-forward"></i></button>
        </div>
    );
  }
  
  export default Input;
  
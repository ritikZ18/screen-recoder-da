declare module '*.css';
declare module '*.scss';

// If you use CSS modules (import styles from './X.module.css'):
declare module '*.module.css' {
  const classes: { [key: string]: string };
  export default classes;
}
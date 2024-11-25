import './style.css';
import Alpine from 'alpinejs';

window.Alpine = Alpine;

Alpine.data('app', () => ({
  count: 0,
}))

Alpine.start();

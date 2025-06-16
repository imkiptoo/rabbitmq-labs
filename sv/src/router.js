import { writable } from 'svelte/store';

export const currentRoute = writable('');

const routes = {
  '/': 'home',
  '/home': 'home',
  '/logger': 'logger',
  '/workers': 'workers',
  '/game': 'game',
  '/rpc': 'rpc',
  '/drawing': 'drawing',
  '/simulator': 'simulator',
  '/exchange-types': 'exchange-types',
  '/newsimulations': 'newsimulations'
};

function updateRoute() {
  const path = window.location.pathname;
  const route = routes[path] || 'home';
  currentRoute.set(route);
}

export function initRouter() {
  updateRoute();
  window.addEventListener('popstate', updateRoute);
  
  return () => {
    window.removeEventListener('popstate', updateRoute);
  };
}

export function navigateTo(route) {
  const path = route === 'home' ? '/' : `/${route}`;
  window.history.pushState({}, '', path);
  updateRoute();
}
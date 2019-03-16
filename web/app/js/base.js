const WHOOSH_DURATION = 1000;
const execAfter = setTimeout;

// #######
// PAGES #
// #######
export const TEMPLATES = {};
let PAGES = {};
export const PAGE = (page, title, template, navActive=undefined)=>{
  let obj = {page,title,template};
  if(navActive !== undefined) obj.navActice = navActive;
  PAGES[page] = obj;
};
PAGES = PAGE;
let NAV_ACTIVE = 'librarium';
let NAV_BAR_PAGES = [
  // PAGE.librarium,
  // PAGE.guilds,
  // PAGE.mybooks,
  // PAGE.aristocracy,
];
export const SETUP_NAVBAR = stuff=>{NAV_BAR_PAGES = stuff};


// ########
// ROUTER #
// ########
export const ROUTER = new Navigo(null, true, '#');
//ROUTER.on('*', (a,b,c)=>console.debug(a,b,c)).resolve();

// TODO delete!
const UNLOATh = ()=>document.querySelector(':root').classList.remove('loading');

ROUTER
  .on(()=>ROUTER.navigate('librarium'))
  .on('librarium', ()=>renderPage(()=>Promise.resolve({}),PAGE.librarium))
  .on('guilds', ()=>{console.warn("TÜDÜ: guilds"),UNLOATh()})
  .on('mybooks', ()=>{console.warn("TÜDÜ: mybooks"),UNLOATh()})
  .on('aristocracy', ()=>renderPage(()=>Promise.resolve({}),PAGE.aristocracy))
  .on('profile', ()=>{console.warn("TÜDÜ: profile"),UNLOATh()});
ROUTER.notFound(()=>{
  const page = ROUTER._lastRouteResolved;
  console.error('Whoopsie! Looks like 404 to me ...', page);
});

// ######
// AUTH #
// ######
const KC_CONF_LOCATION = '../keycloak.json';
const KC_REFRESH_INTERVAL = 5; // seconds -> how often it is checked
const KC_REFRESH_THRESHOLD = 10; // seconds -> remaining time which causes refresh

export let keycloak = null;
let keycloakUpdateInterval = null;

function loadKeycloak(waitForStuff, thenDoStuff) {
  console.debug("Loading keycloak")
  document.querySelector(':root').classList.add('loading');
  if(typeof Keycloak === 'undefined' || !Keycloak){
    axios.get(KC_CONF_LOCATION)
      .then(res => res.data)
      .then(conf => {
        let scriptLocation = `${conf['auth-server-url']}/js/keycloak.js`;
        let scriptNode = document.createElement('script');
        scriptNode.addEventListener('error', errorEvt => {
          console.error('error loading keycloak script', errorEvt)
        });
        scriptNode.addEventListener('load', loadEvt => {
          console.debug('keycloak script loaded!');
          initKeycloak(waitForStuff, thenDoStuff);
        });
        scriptNode.src = scriptLocation;
        scriptNode.async = true;
        document.querySelector('head').appendChild(scriptNode);
      })
      .catch(err => {
        console.error('Fetching Keycloak configuration failed!', err);
      })
  }

}

function initKeycloak(waitForStuff, thenDoStuff){
  if(!keycloak){
    console.debug("Init keycloak")
    // TODO the following seems to be easier than passing the conf object o_O ... we should be able to reuse it!
    keycloak = new Keycloak(KC_CONF_LOCATION);
  }

  keycloak.init({
    onLoad: 'check-sso',
  })
  .success(()=>{
    waitForStuff.then(thenDoStuff);
    updateKeycloakState();
  })
  .error(err => {
    console.error('failed initialising keycloak', err);
  })
}

function updateKeycloakState(){
  if(keycloak && keycloak.authenticated && keycloakUpdateInterval === null){
    keycloakUpdateInterval = setInterval(refreshToken, KC_REFRESH_INTERVAL * 1000)
  } else if(!(keycloak && keycloak.authenticated) && keycloakUpdateInterval !== null){
    clearInterval(keycloakUpdateInterval);
    keycloakUpdateInterval = null;
  }
}

function refreshToken() {
  if(!keycloak) return console.warn("Keycloak, not set");
  keycloak.updateToken(KC_REFRESH_THRESHOLD)
    .success(refreshed => {
      if(refreshed){
        console.debug('keycloak token refreshed');
        updateKeycloakState();
      }
    })
    .error(err => {
        console.err('refreshing token failed:', err);
        updateKeycloakState();
    });
}


// #####
// API #
// #####
export const API = axios.create({
  baseURL: 'http://localhost:8080/v1/',
  timeout: 1000,
  responseType:'json',
});
//inject auth header if not already set and a token is available
API.interceptors.request.use (
  config => {
    if(!config.headers.Authorization && keycloak && keycloak.authenticated){
      config.headers.Authorization = `Bearer ${keycloak.token}`;
    }
    return config;
  },
  error => Promise.reject(error)
);


// #####################
// UI VOODOO FUNCTIONS #
// #####################
function renderPage(loadData, page, args={}) {
  const navPageActive = page.navActice !== undefined ? page.navActice : page.page;
  const root = document.querySelector(':root');
  //loadingScreen
  root.classList.add('loading');
  // query data
  loadData(args).then(data => {
    // render data to template
    const rendered = Mustache.render(TEMPLATES[page.template], data);
    // generate page element
    let pageElement = document.createElement('div');
    pageElement.classList.add('page');
    pageElement.innerHTML = rendered;
    // store old pages
    const oldPages = document.querySelectorAll('main > .page');
    // ... add class "old" to these
    oldPages.forEach(e => e.classList.add('old'));
    // remove loading screen
    root.classList.remove('loading');
    // add new page to main element
    document.querySelector('main').appendChild(pageElement);
    // update navigation bar (maybe a new item is active now‽)
    NAV_ACTIVE = navPageActive;
    updateNavBar();
    // remove old page elements after woosh animation
    execAfter(()=>oldPages.forEach(e => e.remove()), WHOOSH_DURATION);
  }).catch(e => {
    console.error('we got errœr', e);
    root.classList.remove('loading');
  });
}
PAGE._RENDER = renderPage;

function updateNavBar() {
  const newHtml = Mustache.render(TEMPLATES.nav_bar, {
    pages: NAV_ACTIVE ? NAV_BAR_PAGES.map(p => {
      if (p.page === NAV_ACTIVE) {
        return {...p, class:['active']};
      }
      return p;
    }) : NAV_BAR_PAGES,
    auth: keycloak.authenticated,
  });
  document.querySelector('nav.topnav').outerHTML = newHtml;
}


// #####################
// GENREAL MAGIC STUFF #
// #####################
export const MAGIC = (waitForStuff, thenDoStuff)=>{
  loadKeycloak(waitForStuff, thenDoStuff);
};

---
title: Redux
---

# Open Circuits Redux Style Guide!

*Based off of Redux's own style guide*

## Table of Contents

  1. [Essential Rules](#essential-rules)
  1. [Strong Recommendations](#strong-recommendations)

## Essential Rules
- Do not mutate state
  - Mutating state is the most common cause of bugs in Redux applications, including components failing to re-render properly, and will also break time-travel debugging in the Redux DevTools. Actual mutation of state values should always be avoided, both inside reducers and in all other application code.

  - Use tools such as redux-immutable-state-invariant to catch mutations during development, and Immer to avoid accidental mutations in state updates.

  :::note
  It is okay to modify copies of existing values - that is a normal part of writing immutable update logic. Also, if you are using the Immer library for immutable updates, writing "mutating" logic is acceptable because the real data isn't being mutated - Immer safely tracks changes and generates immutably-updated values internally.
  :::

- Reducers must have side effects
  - Reducer functions should only depend on their state and action arguments, and should only calculate and return a new state value based on those arguments. They must not execute any kind of asynchronous logic (AJAX calls, timeouts, promises), generate random values (Date.now(), Math.random()), modify variables outside the reducer, or run other code that affects things outside the scope of the reducer function.

  :::note
  It is acceptable to have a reducer call other functions that are defined outside of itself, such as imports from libraries or utility functions, as long as they follow the same rules.
  :::

- Do Not Put Non-Serializable Values in State or Actions
  - Avoid putting non-serializable values such as Promises, Symbols, Maps/Sets, functions, or class instances into the Redux store state or dispatched actions. This ensures that capabilities such as debugging via the Redux DevTools will work as expected. It also ensures that the UI will update as expected.

  :::note
  You may put non-serializable values in actions if the action will be intercepted and stopped by a middleware before it reaches the reducers. Middleware such as redux-thunk and redux-promise are examples of this.
  :::

- Only One Redux Store Per App
  - A standard Redux application should only have a single Redux store instance, which will be used by the whole application. It should typically be defined in a separate file such as store.js.

  - Ideally, no app logic will import the store directly. It should be passed to a React component tree via <Provider\>, or referenced indirectly via middleware such as thunks. In rare cases, you may need to import it into other logic files, but this should be a last resort.

## Strong Recommendations
- Use Redux Toolkit for Writing Redux Logic
  - Redux Toolkit is our recommended toolset for using Redux. It has functions that build in our suggested best practices, including setting up the store to catch mutations and enable the Redux DevTools Extension, simplifying immutable update logic with Immer, and more.

  - You are not required to use RTK with Redux, and you are free to use other approaches if desired, but using RTK will simplify your logic and ensure that your application is set up with good defaults.

- Use Immer for Writing Immutable Updates​
  - Writing immutable update logic by hand is frequently difficult and prone to errors. Immer allows you to write simpler immutable updates using "mutative" logic, and even freezes your state in development to catch mutations elsewhere in the app. We recommend using Immer for writing immutable update logic, preferably as part of Redux Toolkit.

- Structure Files as Feature Folders with Single-File Logic​
  - Redux itself does not care about how your application's folders and files are structured. However, co-locating logic for a given feature in one place typically makes it easier to maintain that code.

  - Because of this, we recommend that most applications should structure files using a "feature folder" approach (all files for a feature in the same folder). Within a given feature folder, the Redux logic for that feature should be written as a single "slice" file, preferably using the Redux Toolkit createSlice API. (This is also known as the "ducks" pattern). While older Redux codebases often used a "folder-by-type" approach with separate folders for "actions" and "reducers", keeping related logic together makes it easier to find and update that code.

- Put as Much Logic as Possible in Reducers​
  - Wherever possible, try to put as much of the logic for calculating a new state into the appropriate reducer, rather than in the code that prepares and dispatches the action (like a click handler). This helps ensure that more of the actual app logic is easily testable, enables more effective use of time-travel debugging, and helps avoid common mistakes that can lead to mutations and bugs.

  - There are valid cases where some or all of the new state should be calculated first (such as generating a unique ID), but that should be kept to a minimum.

- Reducers Should Own the State Shape​
  - The Redux root state is owned and calculated by the single root reducer function. For maintainability, that reducer is intended to be split up by key/value "slices", with each "slice reducer" being responsible for providing an initial value and calculating the updates to that slice of the state.

  - In addition, slice reducers should exercise control over what other values are returned as part of the calculated state. Minimize the use of "blind spreads/returns" like return action.payload or return {...state, ...action.payload}, because those rely on the code that dispatched the action to correctly format the contents, and the reducer effectively gives up its ownership of what that state looks like. That can lead to bugs if the action contents are not correct.

:::note
A "spread return" reducer may be a reasonable choice for scenarios like editing data in a form, where writing a separate action type for each individual field would be time-consuming and of little benefit.
:::

- Name State Slices Based On the Stored Data​
  - As mentioned in Reducers Should Own the State Shape , the standard approach for splitting reducer logic is based on "slices" of state. Correspondingly, combineReducers is the standard function for joining those slice reducers into a larger reducer function.

  - The key names in the object passed to combineReducers will define the names of the keys in the resulting state object. Be sure to name these keys after the data that is kept inside, and avoid use of the word "reducer" in the key names. Your object should look like {users: {}, posts: {}}, rather than {usersReducer: {}, postsReducer: {}}.

- Organize State Structure Based on Data Types, Not Components​
  - Root state slices should be defined and named based on the major data types or areas of functionality in your application, not based on which specific components you have in your UI. This is because there is not a strict 1:1 correlation between data in the Redux store and components in the UI, and many components may need to access the same data. Think of the state tree as a sort of global database that any part of the app can access to read just the pieces of state needed in that component.

  - For example, a blogging app might need to track who is logged in, information on authors and posts, and perhaps some info on what screen is active. A good state structure might look like {auth, posts, users, ui}. A bad structure would be something like {loginScreen, usersList, postsList}.

- Treat Reducers as State Machines​
  - Many Redux reducers are written "unconditionally". They only look at the dispatched action and calculate a new state value, without basing any of the logic on what the current state might be. This can cause bugs, as some actions may not be "valid" conceptually at certain times depending on the rest of the app logic. For example, a "request succeeded" action should only have a new value calculated if the state says that it's already "loading", or an "update this item" action should only be dispatched if there is an item marked as "being edited".

  - To fix this, treat reducers as "state machines", where the combination of both the current state and the dispatched action determines whether a new state value is actually calculated, not just the action itself unconditionally.

- Normalize Complex Nested/Relational State​
  - Many applications need to cache complex data in the store. That data is often received in a nested form from an API, or has relations between different entities in the data (such as a blog that contains Users, Posts, and Comments).

  - Prefer storing that data in a "normalized" form in the store. This makes it easier to look up items based on their ID and update a single item in the store, and ultimately leads to better performance patterns.

- Keep State Minimal and Derive Additional Values​
  - Whenever possible, keep the actual data in the Redux store as minimal as possible, and derive additional values from that state as needed. This includes things like calculating filtered lists or summing up values. As an example, a todo app would keep an original list of todo objects in state, but derive a filtered list of todos outside the state whenever the state is updated. Similarly, a check for whether all todos have been completed, or number of todos remaining, can be calculated outside the store as well.

 - This has several benefits:

    - The actual state is easier to read
    - Less logic is needed to calculate those additional values and keep them in sync with the rest of the data
    - The original state is still there as a reference and isn't being replaced
  - Deriving data is often done in "selector" functions, which can encapsulate the logic for doing the derived data calculations. In order to improve performance, these selectors can be memoized to cache previous results, using libraries like reselect and proxy-memoize.

- Model Actions as Events, Not Setters​
  - Redux does not care what the contents of the action.type field are - it just has to be defined. It is legal to write action types in present tense ("users/update"), past tense ("users/updated"), described as an event ("upload/progress"), or treated as a "setter" ("users/setUserName"). It is up to you to determine what a given action means in your application, and how you model those actions.

  - However, we recommend trying to treat actions more as "describing events that occurred", rather than "setters". Treating actions as "events" generally leads to more meaningful action names, fewer total actions being dispatched, and a more meaningful action log history. Writing "setters" often results in too many individual action types, too many dispatches, and an action log that is less meaningful.

- Write Meaningful Action Names​
  - The action.type field serves two main purposes:

    - Reducer logic checks the action type to see if this action should be handled to calculate a new state
    - Action types are shown in the Redux DevTools history log for you to read
  - Per Model Actions as "Events", the actual contents of the type field do not matter to Redux itself. However, the type value does matter to you, the developer. Actions should be written with meaningful, informative, descriptive type fields. Ideally, you should be able to read through a list of dispatched action types, and have a good understanding of what happened in the application without even looking at the contents of each action. Avoid using very generic action names like "SET_DATA" or "UPDATE_STORE", as they don't provide meaningful information on what happened.

- Allow Many Reducers to Respond to the Same Action​
  - Redux reducer logic is intended to be split into many smaller reducers, each independently updating their own portion of the state tree, and all composed back together to form the root reducer function. When a given action is dispatched, it might be handled by all, some, or none of the reducers.

  - As part of this, you are encouraged to have many reducer functions all handle the same action separately if possible. In practice, experience has shown that most actions are typically only handled by a single reducer function, which is fine. But, modeling actions as "events" and allowing many reducers to respond to those actions will typically allow your application's codebase to scale better, and minimize the number of times you need to dispatch multiple actions to accomplish one meaningful update.

- Avoid Dispatching Many Actions Sequentially​
  - Avoid dispatching many actions in a row to accomplish a larger conceptual "transaction". This is legal, but will usually result in multiple relatively expensive UI updates, and some of the intermediate states could be potentially invalid by other parts of the application logic. Prefer dispatching a single "event"-type action that results in all of the appropriate state updates at once, or consider use of action batching addons to dispatch multiple actions with only a single UI update at the end.

- Evaluate Where Each Piece of State Should Live​
  - The "Three Principles of Redux" says that "the state of your whole application is stored in a single tree". This phrasing has been over-interpreted. It does not mean that literally every value in the entire app must be kept in the Redux store. Instead, there should be a single place to find all values that you consider to be global and app-wide. Values that are "local" should generally be kept in the nearest UI component instead.

  - Because of this, it is up to you as a developer to decide what state should actually live in the Redux store, and what should stay in component state. Use these rules of thumb to help evaluate each piece of state and decide where it should live.

- Use the React-Redux Hooks API​
  - Prefer using the React-Redux hooks API (useSelector and useDispatch) as the default way to interact with a Redux store from your React components. While the classic connect API still works fine and will continue to be supported, the hooks API is generally easier to use in several ways. The hooks have less indirection, less code to write, and are simpler to use with TypeScript than connect is.

  - The hooks API does introduce some different tradeoffs than connect does in terms of performance and data flow, but we now recommend them as the default.

- Connect More Components to Read Data from the Store​
  - Prefer having more UI components subscribed to the Redux store and reading data at a more granular level. This typically leads to better UI performance, as fewer components will need to render when a given piece of state changes.

  - For example, rather than just connecting a <UserList\> component and reading the entire array of users, have <UserList\> retrieve a list of all user IDs, render list items as <UserListItem userId\ = {userId\}\>, and have <UserListItem\> be connected and extract its own user entry from the store.

  - This applies for both the React-Redux connect() API and the useSelector() hook.

- Use the Object Shorthand Form of mapDispatch with connect​
  - The mapDispatch argument to connect can be defined as either a function that receives dispatch as an argument, or an object containing action creators. We recommend always using the "object shorthand" form of mapDispatch, as it simplifies the code considerably. There is almost never a real need to write mapDispatch as a function.

- Call useSelector Multiple Times in Function Components​
  - When retrieving data using the useSelector hook, prefer calling useSelector many times and retrieving smaller amounts of data, instead of having a single larger useSelector call that returns multiple results in an object. Unlike mapState, useSelector is not required to return an object, and having selectors read smaller values means it is less likely that a given state change will cause this component to render.

  - However, try to find an appropriate balance of granularity. If a single component does need all fields in a slice of the state , just write one useSelector that returns that whole slice instead of separate selectors for each individual field.

- Use Static Typing​
  - Use a static type system like TypeScript or Flow rather than plain JavaScript. The type systems will catch many common mistakes, improve the documentation of your code, and ultimately lead to better long-term maintainability. While Redux and React-Redux were originally designed with plain JS in mind, both work well with TS and Flow. Redux Toolkit is specifically written in TS and is designed to provide good type safety with a minimal amount of additional type declarations.

- Use the Redux DevTools Extension for Debugging​
  - Configure your Redux store to enable debugging with the Redux DevTools Extension. It allows you to view:

    - The history log of dispatched actions
    - The contents of each action
    - The final state after an action was dispatched
    - The diff in the state after an action
    - The function stack trace showing the code where the action was actually dispatched
  - In addition, the DevTools allows you to do "time-travel debugging", stepping back and forth in the action history to see the entire app state and UI at different points in time.

  - Redux was specifically designed to enable this kind of debugging, and the DevTools are one of the most powerful reasons to use Redux.

- Use Plain JavaScript Objects for State​
  - Prefer using plain JavaScript objects and arrays for your state tree, rather than specialized libraries like Immutable.js. While there are some potential benefits to using Immutable.js, most of the commonly stated goals such as easy reference comparisons are a property of immutable updates in general, and do not require a specific library. This also keeps bundle sizes smaller and reduces complexity from data type conversions.

  - As mentioned above, we specifically recommend using Immer if you want to simplify immutable update logic, specifically as part of Redux Toolkit.
import React from 'react';
import { HashRouter, Navigate, Route, Routes } from 'react-router-dom';
import { ToastContainer, toast, Slide } from 'react-toastify';
import 'react-toastify/dist/ReactToastify.css';
import TestPageOne from './Pages/Registration';
import TestPageTwo from './Pages/PrepairingData';
import TestPageThree from './Pages/UpdateData';
import TestPageFour from './Pages/MintNft';
import TestPageFive from './Pages/CreateRevenue';
import TestPageSix from './Pages/AlterRevenue';
import TestPageSeven from './Pages/DeactivateListing';
import CreateGroup from './Pages/CreateGroup';


export default function App() {
/*
  React.useEffect(async () => {
    const fetchObj = await fetch(window.location.origin + window.location.pathname + '/' + 'projectConfig.json')
    .then((response) => response.json())
    .catch((err) => console.error("Error while fetching projectConfig.json: ", err));
    setConfigObj(fetchObj);
  }, [])
*/

  //** RUN `npm run webdev` to start testing! */
  
  return (
    <HashRouter>
      <Routes>
        
        <Route 
          exact
          path='/'
          element={<Navigate to={'/testone'} />}
        />

        <Route 
          exact
          path='/testone'
          element={
            <TestPageOne />
          }
        />
        <Route 
          exact
          path='/testtwo'
          element={
            <TestPageTwo />
          }
        />
        <Route 
          exact
          path='/testthree'
          element={
            <TestPageThree />
          }
        />
        <Route 
          exact
          path='/testfour'
          element={
            <TestPageFour />
          }
        />
        <Route 
          exact
          path='/testfive'
          element={
            <TestPageFive />
          }
        />       
        <Route 
          exact
          path='/testsix'
          element={
            <TestPageSix />
          }
        />     
        <Route 
          exact
          path='/testseven'
          element={
            <TestPageSeven />
          }
        />           
        <Route 
          exact
          path='/create_group'
          element={
            <CreateGroup />
          }
        />        

      </Routes>
    </HashRouter>
  );
}
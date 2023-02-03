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
import Buy from './Pages/Buy';
import IncomeTables from './Pages/IncomeTables';
import Payout from './Pages/Payout';


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
          element={<Navigate to={'/register_user'} />}
        />

        <Route 
          exact
          path='/register_user'
          element={
            <TestPageOne />
          }
        />
        <Route 
          exact
          path='/prepair_data'
          element={
            <TestPageTwo />
          }
        />
        <Route 
          exact
          path='/update_data'
          element={
            <TestPageThree />
          }
        />
        <Route 
          exact
          path='/do_the_minting'
          element={
            <TestPageFour />
          }
        />
        <Route 
          exact
          path='/create_revenue_table'
          element={
            <TestPageFive />
          }
        />       
        <Route 
          exact
          path='/alter_revenue_table'
          element={
            <TestPageSix />
          }
        />     
        <Route 
          exact
          path='/deactivate_listing'
          element={
            <TestPageSeven />
          }
        />           
        <Route
          exact
          path='/buy'
          element={
            <Buy />
          }
        />
        <Route
          exact
          path='/income'
          element={
            <IncomeTables />

          }
        />
        <Route 
          exact
          path='/create_group'
          element={
            <CreateGroup />
          }
        />        
        <Route 
          exact
          path='/payout'
          element={
            <Payout />
          }
        /> 

      </Routes>
    </HashRouter>
  );
}
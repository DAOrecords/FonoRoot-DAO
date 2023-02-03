import React from 'react';
import { Link } from 'react-router-dom';
import { login, logout } from './utils';


export default function Nav() {
  return (
    <nav>
      <ul id="navList">
        <li className="navListElement">
          <Link to={'/register_user'}>Registration</Link>
        </li>

        <li className="navListElement">
          <Link to={'/prepair_data'}>Prepairing Data</Link>
        </li>

        <li className="navListElement">
          <Link to={'/update_data'}>Updating Data</Link>
        </li>

        <li className="navListElement">
          <Link to={'/do_the_minting'}>Minting</Link>
        </li>

        <li className="navListElement">
          <Link to={'/create_revenue_table'}>Create Revenue Table</Link>
        </li>

        <li className="navListElement">
          <Link to={'/alter_revenue_table'}>Alter Revenue Table</Link>
        </li>

        <li className="navListElement">
          <Link to={'/deactivate_listing'}>Deactivate Listing</Link>
        </li>

        <li className="navListElement">
          <Link to={'/buy'}>Buy</Link>  
        </li>

        <li className="navListElement">
          <Link to={'/income'}>List IncomeTables</Link>
        </li>

        <li className="navListElement">
          <Link to={'/payout'}>Payout</Link>
        </li>
        
        <li className="navListElement">
          <Link to={'/create_group'}>Create New Group</Link>
        </li>
      </ul>
      {(window.accountId) ? <button onClick={logout}>Logout</button> : <button onClick={login}>Login</button>}
    </nav>
  )
}

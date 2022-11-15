import React from 'react';
import { Link } from 'react-router-dom';
import { login, logout } from './utils';


export default function Nav() {
  return (
    <nav>
      <ul id="navList">
        <li className="navListElement">
          <Link to={'/testone'}>Registration</Link>
        </li>

        <li className="navListElement">
          <Link to={'/testtwo'}>Prepairing Data</Link>
        </li>

        <li className="navListElement">
          <Link to={'/testthree'}>Updating Data</Link>
        </li>

        <li className="navListElement">
          <Link to={'/testfour'}>Minting</Link>
        </li>

        <li className="navListElement">
          <Link to={'/testfive'}>Create Revenue Table</Link>
        </li>

        <li className="navListElement">
          <Link to={'/testsix'}>Alter Revenue Table</Link>
        </li>

        <li className="navListElement">
          <Link to={'/testseven'}>Deactivate Listing</Link>
        </li>
        <li className="navListElement">
          <Link to={'/create_group'}>Create New Group</Link>
        </li>
      </ul>
      {(window.accountId) ? <button onClick={logout}>Logout</button> : <button onClick={login}>Login</button>}
    </nav>
  )
}

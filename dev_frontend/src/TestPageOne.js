import React, { useState } from 'react';
import Nav from './Nav';


export default function TestPageOne() {
  const [newUser, setNewUser] = useState("");
  const [contractName, setContractName] = useState("");
  const [isMaster, setIsMaster] = useState(false);
  const [groupName, setGroupName] = useState("");
  const [resultVisible, setResultVisible] = useState(false);

  function doRegisterAction() {
    const prefix = isMaster ? "master" : "collab";
    const GroupName = prefix + "_" + contractName;
    setGroupName(GroupName);
    setResultVisible(true);
  }


  return (
    <>
      <Nav />
      <main>
        <h2>Registration Proccess</h2>

        <section>
          <p>{"First the User registers. Indeed, the User can not complete the registration proccess, it is done by us, members of the Council / Admins."}</p>
          <p>{"The easiest thing would be to add the User to the correct Group (Role)."}</p>
          <p>{"For example, if we want to enable him to mint on "}<strong>nft.metajax.near</strong>{", we add him to "}<code>master_nft.metajax.near.</code></p>
          <p>{"If we want to add him with lower rights, we add him/her to "}<code>collab_nft.metajax.near.</code></p>

          <label>Can Mint?</label>
          <input type={"checkbox"} checked={isMaster} onChange={(e) => setIsMaster(e.target.checked)}></input>
          <input value={newUser} onChange={(e) => setNewUser(e.target.value)} placeholder={"Account Name"}></input>
          <input value={contractName} onChange={(e) => setContractName(e.target.value)} placeholder={"Contract"}></input>
          <button onClick={doRegisterAction}>Register User</button>

          {resultVisible && (
            <div>
              <p>
                <strong>This user would be added: </strong>
                <code>{newUser}</code>
              </p>
              <p>
                <strong>User would be added to this group: </strong>
                <code>{groupName}</code>
              </p>
            </div>
          )}
        </section>

      </main>
    </>
  )
}

import React, { useState, useEffect } from 'react';
import Nav from '../Nav';
import { actOnProposal, getLastProposalId, getListOfPolicyRoles, getListOfProposals, registerUser } from '../utils';


export default function TestPageOne() {
  const [newUser, setNewUser] = useState("");
  const [contractName, setContractName] = useState("");
  const [isMaster, setIsMaster] = useState(true);
  const [groupName, setGroupName] = useState("");
  const [resultVisible, setResultVisible] = useState(false);
  const [proposalId, setProposalId] = useState(null);
  const [masterGroups, setMasterGroups] = useState([]);

  // Create proposal for Registration
  async function createProposal() {
    const prefix = isMaster ? "master" : "collab";
    const GroupName = prefix + "_" + contractName;
    setGroupName(GroupName);
    setResultVisible(true);

    const lastProposalId =  await getLastProposalId();
    localStorage.setItem("last_proposal_id", lastProposalId);
    localStorage.setItem("group_name_when_registering", GroupName);

    registerUser(newUser, GroupName);
  }

  // Act on proposal for the Registration of New User
  function finalizeRegistration() {
    actOnProposal(proposalId);
  }

  // This useEffect is setting the proposalId, for finalizing
  useEffect(async () => {
    const index = localStorage.getItem("last_proposal_id") - 2;
    const savedGroupName = localStorage.getItem("group_name_when_registering");
    const proposalList = await getListOfProposals(index);
    console.log("proposalList: ", proposalList);

    const inProgressProposals = proposalList.filter((proposalEntry) => proposalEntry.status === "InProgress");
    console.log("inProgressProposal: ", inProgressProposals);
    if (inProgressProposals.length === 0) return;

    const addMemberKindProposals = inProgressProposals.filter((proposalEntry) => proposalEntry.kind.hasOwnProperty("AddMemberToRole"));
    console.log("addMemberKindProposals: ", addMemberKindProposals);

    const i = addMemberKindProposals.findIndex((proposalEntry) => proposalEntry.kind.AddMemberToRole.role === savedGroupName);
    console.log("the Entry: ", inProgressProposals[i]);

    const theId = addMemberKindProposals[i].id;
    setProposalId(theId);

  }, []);

  // Fetching the Master Groups
  useEffect(async () => {
    const allRoles = await getListOfPolicyRoles();
    console.log("All roles: ", allRoles);

    const mGroups = allRoles.filter((role) => role.name.includes("master_"));
    console.log("Master Groups: ", mGroups);

    setMasterGroups(mGroups);

  }, []);


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
          <input type={"checkbox"} checked={isMaster} onChange={(e) => setIsMaster(e.target.checked)} disabled={true}></input>
          <input value={newUser} onChange={(e) => setNewUser(e.target.value)} placeholder={"Account Name"}></input>
          <input value={contractName} onChange={(e) => setContractName(e.target.value)} placeholder={"Contract"}></input>
          <button onClick={createProposal}>Create Register User Proposal</button>

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

        <section>
          <p>{"We will need to act on the proposal that we've just created. For that, we need the "}<code>{"last_proposal_id"}</code>{", and we need to get the proposals from that index, or from before that index, let's say 10."}</p>
          <p>{"We need to find that proposal that we've just created."}</p>
          <p>{"Last proposal ID: "} {localStorage.getItem("last_proposal_id")}</p>
          <p>{"We are looking for a proposal that is "}<code>{"InProgress"}</code>{", and the contract name is the previously set contract name prefixed with "}<code>{"master_"}</code></p>
          <p>{"And proposal kind is "}<code>{"AddMemberToRole"}</code></p>
          <p>{"We save the contract name to LocalStorage as well."}</p>
          <p>{". "}</p>
          <p>{"The ID of the proposal that we want to act on should be: "}<code>{proposalId}</code></p>
          <button onClick={finalizeRegistration}>Finalize Registration</button>
        </section>

        <section>
        <p>{"These are the accounts which can do Mint, on specific contract:"}</p>
          <ul>
            {masterGroups.map((masterGroup) => (
              <li key={masterGroup.name}>
                <code>{masterGroup.name}</code>
                <ul>
                  {masterGroup.kind.Group.map((artist) => (
                    <li key={masterGroup.name + artist}>{artist}</li>
                  ))}
                </ul>
              </li>
            ))}
          </ul>
        </section>

      </main>
    </>
  )
}

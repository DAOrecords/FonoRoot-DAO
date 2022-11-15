import React, { useState, useEffect } from 'react';
import Nav from '../Nav';
import { actOnProposal, createGroup, getLastProposalId, getListOfPolicyRoles, getListOfProposals } from '../utils';


export default function CreateGroup() {
  const [contractName, setContractName] = useState("");
  const [proposalId, setProposalId] = useState(null);
  const [masterGroups, setMasterGroups] = useState([]);

  async function createNewGroupProposal() {
    const isMaster = true;
    const prefix = isMaster ? "master" : "collab";
    const groupName = prefix + "_" + contractName;

    const lastProposalId = await getLastProposalId();
    localStorage.setItem("last_proposal_id", lastProposalId);
    localStorage.setItem("name_of_the_new_group", groupName);

    createGroup(groupName);
  }

  // Act on proposal for the Creation of New Group
  function finalizeCreateNewGroup() {
    actOnProposal(proposalId);
  }

  useEffect(async () => {
    const index = localStorage.getItem("last_proposal_id") - 2;
    const savedGroupName = localStorage.getItem("name_of_the_new_group");
    const proposalList = await getListOfProposals(index);
    console.log("proposalList: ", proposalList);

    const inProgressProposals = proposalList.filter((proposalEntry) => proposalEntry.status === "InProgress");
    console.log("inProgressProposal: ", inProgressProposals);
    if (inProgressProposals.length === 0) return;

    const changePolicyKindProposals = inProgressProposals.filter((proposalEntry) => proposalEntry.kind.hasOwnProperty("ChangePolicyAddOrUpdateRole"));
    console.log("changePolicyKindProposals: ", changePolicyKindProposals);


    const i = changePolicyKindProposals.findIndex((proposalEntry) => {
      console.log("proposalEntry.kind.ChangePolicyAddOrUpdateRole.role.name", proposalEntry.kind.ChangePolicyAddOrUpdateRole.role.name)
      console.log("savedGroupName: ", savedGroupName)
      return proposalEntry.kind.ChangePolicyAddOrUpdateRole.role.name === savedGroupName
    });
    console.log("the Entry: ", inProgressProposals[i]);

    const theId = changePolicyKindProposals[i].id;
    setProposalId(theId);

  }, []);

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
        <h2>Create New Group</h2>

        <section>
          <p>{"Master Group has to exist for each contract, so we can add Artists to it."}</p>
          <p>{"It is possible, that a group only contains 1 Artist, that would sort of mean that the Artist is owning that minting contract."}</p>
          <p>{"In parallel, we could have contracts as well that could be used by multiple people."}</p>

          <input value={contractName} onChange={(e) => setContractName(e.target.value)} placeholder={"Contract Name"}></input>
          <button onClick={createNewGroupProposal}>Create New Master Group</button>
        </section>

        <section>
          <p>{"We will need to act on the proposal that we've just created. For that, we need the "}<code>{"last_proposal_id"}</code>{", and we need to get the proposals from that index, or from before that index, let's say 10."}</p>
          <p>{"We need to find that proposal that we've just created."}</p>
          <p>{"Last proposal ID: "} {localStorage.getItem("last_proposal_id")}</p>
          <p>{"We are looking for a proposal that is "}<code>{"InProgress"}</code>{", and the contract name is the previously set contract name prefixed with "}<code>{"master_"}</code></p>
          <p>{"And proposal kind is "}<code>{"ChangePolicyAddOrUpdateRole"}</code></p>
          <p>{"We save the contract name to LocalStorage as well."}</p>
          <p>{". "}</p>
          <p>{"The ID of the proposal that we want to act on should be: "}<code>{proposalId}</code></p>
          <button onClick={finalizeCreateNewGroup}>Finalize Creation of New Group</button>
        </section>

        <section>
          <p>{"This are the groups that exist with "}<code>{"master_"}</code>{" prefix:"}</p>
          <ul>
            {masterGroups.map((masterGroup) => (
              <li key={masterGroup.name}><code>{masterGroup.name}</code></li>
            ))}
          </ul>
        </section>
      </main>
    </>
  )
}

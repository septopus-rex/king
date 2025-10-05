import * as anchor from "@coral-xyz/anchor";
import { Pool } from "../target/types/pool";

const program = anchor.workspace.Pool as anchor.Program<Pool>;
const provider = anchor.AnchorProvider.env();

anchor.setProvider(provider);

const reqs={
    
}
package ga

import scala.util.Random

object Chromosome {
  def generateChromosome(numGenes: Int): Chromosome = new Chromosome(Random.alphanumeric.slice(0, numGenes).toList)    
}
  
class Chromosome(genes: List[Char]) {
  def getGenes = genes
    
  def mutate(perSiteProb: Float): Chromosome = 
    new Chromosome(getGenes map ((c: Char) => if (Random.nextFloat < perSiteProb) Random.alphanumeric(0) else c))
    
  def recombine(that: Chromosome): Chromosome =
    Random.nextInt(genes.length) match {
      case pivot => new Chromosome(genes.slice(0, pivot) ++ that.getGenes.slice(pivot, that.getGenes.length))
    }
  
  override def toString = genes.toString
}